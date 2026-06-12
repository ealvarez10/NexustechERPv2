//! CLI del transpilador.
//!
//! ```text
//! odoo2rs models  models/*.py --module sale          # → IR JSON de modelos
//! odoo2rs views   views/*.xml                        # → IR JSON de vistas
//! odoo2rs gen-rust models/*.py --module sale -o out/ # → fragmentos nexus-orm
//! odoo2rs gen-js  views/*.xml -o out/                # → páginas web/src
//! odoo2rs addon   /ruta/al/addon -o generated/       # → todo lo anterior
//! ```

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use odoo2rs::codegen::{js_gen, rust_gen};
use odoo2rs::ir::{ModelIr, ViewBundle};
use odoo2rs::py;
use odoo2rs::xml;

#[derive(Parser)]
#[command(
    name = "odoo2rs",
    version,
    about = "Transpilador Odoo → NexusTech ERP v2 (FASE 1+2: .py/.xml → OdooIR → Rust/JS)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Extrae modelos de archivos/carpetas .py → IR JSON
    /// (consumible por nexus_orm::ir::parse_ir / register_ir_json).
    Models {
        /// Archivos .py o directorios a recorrer.
        paths: Vec<PathBuf>,
        /// Nombre del módulo Odoo de origen (campo `module` del IR).
        #[arg(long)]
        module: Option<String>,
        /// Escribe a archivo en vez de stdout.
        #[arg(long, short)]
        out: Option<PathBuf>,
        /// JSON compacto (por defecto: pretty).
        #[arg(long)]
        compact: bool,
    },
    /// Extrae vistas/acciones/menús de archivos .xml → IR JSON.
    Views {
        paths: Vec<PathBuf>,
        #[arg(long, short)]
        out: Option<PathBuf>,
        #[arg(long)]
        compact: bool,
    },
    /// Genera fragmentos Rust (nexus-orm ModelFragment) desde .py.
    GenRust {
        paths: Vec<PathBuf>,
        #[arg(long)]
        module: Option<String>,
        /// Directorio de salida (un .rs por modelo).
        #[arg(long, short, default_value = "generated/rust")]
        out_dir: PathBuf,
    },
    /// Genera páginas JS (descriptores de vista) desde .xml.
    GenJs {
        paths: Vec<PathBuf>,
        #[arg(long, short, default_value = "generated/js")]
        out_dir: PathBuf,
    },
    /// Procesa un addon completo (__manifest__.py, models/, views/).
    Addon {
        /// Carpeta raíz del addon Odoo.
        dir: PathBuf,
        #[arg(long, short, default_value = "generated")]
        out_dir: PathBuf,
    },
}

fn main() -> Result<()> {
    match Cli::parse().cmd {
        Cmd::Models {
            paths,
            module,
            out,
            compact,
        } => {
            let (models, warnings) = extract_all_models(&paths, module.as_deref())?;
            report(&warnings);
            emit_json(&models, out.as_deref(), compact)?;
            eprintln!("odoo2rs: {} fragmento(s) de modelo extraído(s)", models.len());
        }
        Cmd::Views { paths, out, compact } => {
            let (bundle, warnings) = extract_all_views(&paths)?;
            report(&warnings);
            emit_json(&bundle, out.as_deref(), compact)?;
            eprintln!(
                "odoo2rs: {} vista(s), {} acción(es), {} menú(s)",
                bundle.views.len(),
                bundle.actions.len(),
                bundle.menus.len()
            );
        }
        Cmd::GenRust {
            paths,
            module,
            out_dir,
        } => {
            gen_rust(&paths, module.as_deref(), &out_dir)?;
        }
        Cmd::GenJs { paths, out_dir } => {
            gen_js(&paths, &out_dir)?;
        }
        Cmd::Addon { dir, out_dir } => {
            addon(&dir, &out_dir)?;
        }
    }
    Ok(())
}

// ─── Comandos ───────────────────────────────────────────────────────────

fn extract_all_models(
    paths: &[PathBuf],
    module: Option<&str>,
) -> Result<(Vec<ModelIr>, Vec<String>)> {
    let mut models = Vec::new();
    let mut warnings = Vec::new();
    for file in collect_files(paths, "py")? {
        // __init__.py / __manifest__.py no contienen modelos.
        if file
            .file_name()
            .is_some_and(|n| n.to_string_lossy().starts_with("__"))
        {
            continue;
        }
        let src = fs::read_to_string(&file)
            .with_context(|| format!("leyendo {}", file.display()))?;
        let mut ex = py::extract_models(&src, &file.display().to_string(), module)?;
        models.append(&mut ex.models);
        warnings.append(&mut ex.warnings);
    }
    Ok((models, warnings))
}

fn extract_all_views(paths: &[PathBuf]) -> Result<(ViewBundle, Vec<String>)> {
    let mut bundle = ViewBundle::default();
    let mut warnings = Vec::new();
    for file in collect_files(paths, "xml")? {
        let src = fs::read_to_string(&file)
            .with_context(|| format!("leyendo {}", file.display()))?;
        let mut ex = xml::extract_views(&src, &file.display().to_string())?;
        bundle.views.append(&mut ex.bundle.views);
        bundle.actions.append(&mut ex.bundle.actions);
        bundle.menus.append(&mut ex.bundle.menus);
        warnings.append(&mut ex.warnings);
    }
    Ok((bundle, warnings))
}

fn gen_rust(paths: &[PathBuf], module: Option<&str>, out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir)?;
    let mut count = 0usize;
    for file in collect_files(paths, "py")? {
        if file
            .file_name()
            .is_some_and(|n| n.to_string_lossy().starts_with("__"))
        {
            continue;
        }
        let src = fs::read_to_string(&file)
            .with_context(|| format!("leyendo {}", file.display()))?;
        let origin = file.display().to_string();
        let ex = py::extract_models(&src, &origin, module)?;
        report(&ex.warnings);
        for m in &ex.models {
            let code = rust_gen::fragment_rs(m, &origin);
            let fname = format!(
                "{}{}.rs",
                m.model.replace('.', "_"),
                if m.inherit { "_ext" } else { "" }
            );
            let dest = out_dir.join(&fname);
            fs::write(&dest, code).with_context(|| format!("escribiendo {}", dest.display()))?;
            eprintln!("odoo2rs: {} → {}", m.model, dest.display());
            count += 1;
        }
    }
    eprintln!("odoo2rs: {count} fragmento(s) Rust generado(s) en {}", out_dir.display());
    Ok(())
}

fn gen_js(paths: &[PathBuf], out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir)?;
    let (bundle, warnings) = extract_all_views(paths)?;
    report(&warnings);
    for v in &bundle.views {
        let dest = out_dir.join(js_gen::js_file_name(v));
        fs::write(&dest, js_gen::view_js(v))
            .with_context(|| format!("escribiendo {}", dest.display()))?;
        eprintln!(
            "odoo2rs: {} ({}) → {}",
            v.model.as_deref().unwrap_or("?"),
            v.view_type,
            dest.display()
        );
    }
    eprintln!("odoo2rs: {} página(s) JS generada(s) en {}", bundle.views.len(), out_dir.display());
    Ok(())
}

/// Pipeline completo sobre un addon: manifiesto + models/ + views/.
fn addon(dir: &Path, out_dir: &Path) -> Result<()> {
    let module = dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "addon".into());
    fs::create_dir_all(out_dir)?;

    // __manifest__.py (opcional).
    let manifest_path = dir.join("__manifest__.py");
    if manifest_path.exists() {
        let src = fs::read_to_string(&manifest_path)?;
        let m = py::parse_manifest(&src, &manifest_path.display().to_string())?;
        fs::write(
            out_dir.join(format!("{module}.manifest.json")),
            serde_json::to_string_pretty(&m)?,
        )?;
        eprintln!(
            "odoo2rs: manifiesto '{}' (depends: {})",
            m.name.as_deref().unwrap_or(&module),
            m.depends.join(", ")
        );
    }

    // Modelos: models/ si existe, si no todo el addon.
    let models_dir = dir.join("models");
    let py_root = if models_dir.is_dir() { models_dir } else { dir.to_path_buf() };
    let (models, warns) = extract_all_models(std::slice::from_ref(&py_root), Some(&module))?;
    report(&warns);
    fs::write(
        out_dir.join(format!("{module}.models.json")),
        serde_json::to_string_pretty(&models)?,
    )?;
    gen_rust(&[py_root], Some(&module), &out_dir.join("rust"))?;

    // Vistas: views/ si existe, si no todo el addon.
    let views_dir = dir.join("views");
    let xml_root = if views_dir.is_dir() { views_dir } else { dir.to_path_buf() };
    let (bundle, warns) = extract_all_views(std::slice::from_ref(&xml_root))?;
    report(&warns);
    fs::write(
        out_dir.join(format!("{module}.views.json")),
        serde_json::to_string_pretty(&bundle)?,
    )?;
    gen_js(&[xml_root], &out_dir.join("js"))?;

    eprintln!(
        "odoo2rs: addon '{module}' procesado — {} modelo(s), {} vista(s) → {}",
        models.len(),
        bundle.views.len(),
        out_dir.display()
    );
    Ok(())
}

// ─── Utilería ───────────────────────────────────────────────────────────

/// Expande archivos y directorios (recursivo) filtrando por extensión.
fn collect_files(paths: &[PathBuf], ext: &str) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for p in paths {
        walk(p, ext, &mut out)?;
    }
    out.sort();
    Ok(out)
}

fn walk(p: &Path, ext: &str, out: &mut Vec<PathBuf>) -> Result<()> {
    if p.is_dir() {
        for entry in fs::read_dir(p).with_context(|| format!("leyendo {}", p.display()))? {
            walk(&entry?.path(), ext, out)?;
        }
    } else if p.extension().is_some_and(|e| e == ext) {
        out.push(p.to_path_buf());
    }
    Ok(())
}

fn emit_json<T: serde::Serialize>(value: &T, out: Option<&Path>, compact: bool) -> Result<()> {
    let json = if compact {
        serde_json::to_string(value)?
    } else {
        serde_json::to_string_pretty(value)?
    };
    match out {
        Some(p) => {
            if let Some(parent) = p.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent)?;
                }
            }
            fs::write(p, json).with_context(|| format!("escribiendo {}", p.display()))?;
            eprintln!("odoo2rs: IR escrito en {}", p.display());
        }
        None => println!("{json}"),
    }
    Ok(())
}

fn report(warnings: &[String]) {
    for w in warnings {
        eprintln!("odoo2rs[aviso]: {w}");
    }
}
