import os
import re
import json

addons_dir = "/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/"
output_file = "/home/ealvarez/workspace/NexustechERPv2/model_views_cache.json"

os.makedirs(os.path.dirname(output_file), exist_ok=True)

cache = {}

record_re = re.compile(r'<record\s+[^>]*model="ir\.ui\.view"[^>]*>(.*?)</record>', re.DOTALL)
model_name_re = re.compile(r'_name\s*=\s*[\'"]([a-zA-Z0-9_\.]+)[\'"]')
model_re = re.compile(r'<field\s+[^>]*name="model"[^>]*>(.*?)</field>', re.DOTALL)
field_re = re.compile(r'<field\s+[^>]*name="([a-zA-Z0-9_]+)"', re.DOTALL)

TYPE_MAP = {
    'Char': 'char',
    'Text': 'text',
    'Html': 'html',
    'Boolean': 'boolean',
    'Integer': 'integer',
    'Float': 'float',
    'Monetary': 'monetary',
    'Date': 'date',
    'Datetime': 'datetime',
    'Selection': 'selection',
    'Many2one': 'many2one',
    'One2many': 'one2many',
    'Many2many': 'many2many',
    'Binary': 'binary',
}

def extract_arch(record_body):
    match = re.search(r'<field\s+[^>]*name="arch"[^>]*>', record_body)
    if not match:
        return None
    start_idx = match.end()
    
    depth = 1
    current_idx = start_idx
    while depth > 0 and current_idx < len(record_body):
        next_open = record_body.find('<field', current_idx)
        next_close = record_body.find('</field>', current_idx)
        
        if next_close == -1:
            break
            
        if next_open != -1 and next_open < next_close:
            next_gt = record_body.find('>', next_open)
            if next_gt != -1:
                tag_content = record_body[next_open:next_gt]
                if tag_content.strip().endswith('/'):
                    current_idx = next_gt + 1
                    continue
            
            depth += 1
            current_idx = next_open + 6
        else:
            depth -= 1
            if depth == 0:
                return record_body[start_idx:next_close].strip()
            current_idx = next_close + 8
    return None

def postprocess_view_arch(arch_xml, model_fields, view_type):
    declared = set(re.findall(r'<field\s+[^>]*name="([a-zA-Z0-9_]+)"', arch_xml))
    expr_attrs = re.findall(r'(?:invisible|readonly|required|column_invisible|domain|context|attrs|decoration-[a-z-]+)="([^"]+)"', arch_xml)
    
    referenced = set()
    for expr in expr_attrs:
        tokens = re.findall(r'\b([a-zA-Z0-9_]+)\b', expr)
        for t in tokens:
            if t in model_fields and t not in declared:
                referenced.add(t)
                
    if not referenced:
        return arch_xml
        
    root_match = re.match(r'^<([a-zA-Z0-9_]+)([^>]*)>', arch_xml)
    if root_match:
        insertion_point = root_match.end()
        
        injections = []
        for f in sorted(list(referenced)):
            if view_type == 'list':
                injections.append(f'<field name="{f}" column_invisible="True"/>')
            else:
                injections.append(f'<field name="{f}" invisible="1"/>')
                
        injection_str = "\n" + "\n".join(injections)
        return arch_xml[:insertion_point] + injection_str + arch_xml[insertion_point:]
        
    return arch_xml

# 1. Parse Python files to get exact types & relations
python_models = {}
for root, dirs, files in os.walk(addons_dir):
    for file in files:
        if not file.endswith(".py"):
            continue
        filepath = os.path.join(root, file)
        try:
            with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
                content = f.read()
            
            classes = content.split("class ")
            for cls_content in classes[1:]:
                model_match = model_name_re.search(cls_content)
                if not model_match:
                    continue
                model_name = model_match.group(1).strip()
                
                if model_name not in python_models:
                    python_models[model_name] = {}
                    
                for line in cls_content.split("\n"):
                    field_match = re.match(r'^\s*([a-zA-Z0-9_]+)\s*=\s*fields\.([a-zA-Z0-9_]+)\((.*)', line)
                    if field_match:
                        field_name = field_match.group(1)
                        py_type = field_match.group(2)
                        args_str = field_match.group(3)
                        
                        odoo_type = TYPE_MAP.get(py_type, 'char')
                        
                        relation = None
                        if odoo_type in ('many2one', 'one2many', 'many2many'):
                            rel_match = re.search(r'^[\'"]([a-zA-Z0-9_\.]+)[\'"]', args_str.strip())
                            if rel_match:
                                relation = rel_match.group(1)
                                
                        string_label = field_name.replace('_', ' ').title()
                        string_match = re.search(r'string\s*=\s*[\'"]([^\'"]+)[\'"]', args_str)
                        if string_match:
                            string_label = string_match.group(1)
                            
                        python_models[model_name][field_name] = {
                            "type": odoo_type,
                            "string": string_label,
                        }
                        if relation:
                            python_models[model_name][field_name]["relation"] = relation
        except Exception:
            pass

# 2. Parse XML views and build cache
for root, dirs, files in os.walk(addons_dir):
    for file in files:
        if not file.endswith(".xml"):
            continue
        filepath = os.path.join(root, file)
        try:
            with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
                content = f.read()
            
            for record_match in record_re.finditer(content):
                record_body = record_match.group(1)
                
                model_match = model_re.search(record_body)
                if not model_match:
                    continue
                model_name = model_match.group(1).strip()
                
                arch_xml = extract_arch(record_body)
                if not arch_xml:
                    continue
                
                tag_match = re.search(r'<([a-zA-Z0-9_]+)', arch_xml)
                if not tag_match:
                    continue
                view_type = tag_match.group(1).strip()
                if view_type == 'tree':
                    view_type = 'list'
                
                if view_type not in ('list', 'form', 'kanban', 'search'):
                    continue
                
                fields_in_view = field_re.findall(arch_xml)
                fields_set = set(fields_in_view)
                
                # Retrieve fields from python models, and fall back to XML views
                py_fields = python_models.get(model_name, {})
                
                model_fields = {}
                for f in fields_set:
                    if f in py_fields:
                        model_fields[f] = py_fields[f]
                    else:
                        # Fallback type inference
                        ftype = 'char'
                        if f == 'active' or f.startswith('is_') or f.startswith('has_'):
                            ftype = 'boolean'
                        elif f.endswith('_ids'):
                            ftype = 'one2many'
                        elif f.endswith('_id'):
                            ftype = 'many2one'
                        elif 'date' in f:
                            ftype = 'datetime'
                        elif f.endswith('_time') or f.endswith('_hours') or 'duration' in f or 'amount' in f or 'price' in f or 'qty' in f or 'rate' in f:
                            ftype = 'float'
                        elif f in ('sequence', 'color', 'id') or f.endswith('_count'):
                            ftype = 'integer'
                        elif 'image' in f or 'logo' in f or 'avatar' in f or 'file' in f:
                            ftype = 'binary'
                            
                        relation = None
                        if ftype in ('many2one', 'one2many', 'many2many'):
                            if f.endswith('_id'):
                                relation_base = f[:-3]
                            elif f.endswith('_ids'):
                                relation_base = f[:-4]
                            else:
                                relation_base = f
                            relation = relation_base.replace('_', '.')
                            if relation == 'tag' and model_name.startswith('slide.'):
                                relation = 'slide.channel.tag'
                            elif relation == 'user':
                                relation = 'res.users'
                            elif relation == 'partner':
                                relation = 'res.partner'
                            elif relation == 'company':
                                relation = 'res.company'
                            elif relation == 'currency':
                                relation = 'res.currency'
                            elif relation == 'slide':
                                relation = 'slide.slide'
                                
                        field_entry = {
                            "type": ftype,
                            "string": f.replace('_', ' ').title()
                        }
                        if relation:
                            field_entry["relation"] = relation
                        model_fields[f] = field_entry
                
                # Merge Python fields that are not in the view but might be needed
                for f, fdef in py_fields.items():
                    if f not in model_fields:
                        model_fields[f] = fdef
                
                # Postprocess XML to inject referenced but undeclared fields
                arch_xml = postprocess_view_arch(arch_xml, set(model_fields.keys()), view_type)
                
                if model_name not in cache:
                    cache[model_name] = {
                        "fields": {},
                        "views": {}
                    }
                
                # Update model fields list with metadata
                cache[model_name]["fields"].update(model_fields)
                
                existing_view = cache[model_name]["views"].get(view_type, "")
                if len(arch_xml) > len(existing_view):
                    cache[model_name]["views"][view_type] = arch_xml
                
        except Exception as e:
            pass

# Write JSON
with open(output_file, "w", encoding="utf-8") as f:
    json.dump(cache, f, indent=2)

print(f"Generated cache with {len(cache)} models.")
