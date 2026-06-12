from odoo import api, fields, models


class DemoOrder(models.Model):
    _name = 'demo.order'
    _description = 'Orden demo'
    _order = 'id desc'

    name = fields.Char('Referencia', required=True, default='Nuevo')
    partner_id = fields.Many2one('res.partner', string='Cliente', required=True)
    state = fields.Selection([
        ('draft', 'Borrador'),
        ('done', 'Hecha'),
    ], string='Estado', default='draft')
    line_ids = fields.One2many('demo.order.line', 'order_id', string='Líneas')
    amount_total = fields.Monetary(string='Total', compute='_compute_total', store=True)
    partner_vat = fields.Char(related='partner_id.vat', string='RFC')
    draft_note = fields.Text(store=False)

    @api.depends('line_ids.subtotal')
    def _compute_total(self):
        for order in self:
            order.amount_total = sum(order.line_ids.mapped('subtotal'))

    def action_done(self):
        self.write({'state': 'done'})
        return True
