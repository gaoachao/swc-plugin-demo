use swc_core::{
    common::errors::HANDLER,
    ecma::{
        ast::*,
        visit::{visit_mut_pass, VisitMut, VisitMutWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_call_expr(&mut self, n: &mut CallExpr) {
        match &mut n.callee {
            Callee::Expr(e) => match &**e {
                Expr::Member(m) => match &*m.obj {
                    Expr::This(_) => match &m.prop {
                        MemberProp::Ident(id) => match id.sym.to_string().as_str() {
                            "createSelectorQuery" => {
                                HANDLER.with(|handler| {
                                    handler
                                        .struct_span_warn(
                                            n.span,
                                            format!(
                                                "WARNING: {} is deprecated.",
                                                id.sym.to_string()
                                            )
                                            .as_str(),
                                        )
                                        .emit()
                                });
                            }
                            "getElementById" => {
                                HANDLER.with(|handler| {
                                    handler
                                        .struct_span_warn(
                                            n.span,
                                            format!(
                                                "WARNING: {} is deprecated.",
                                                id.sym.to_string()
                                            )
                                            .as_str(),
                                        )
                                        .emit()
                                });
                            }
                            &_ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        n.visit_mut_children_with(self);
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.apply(&mut visit_mut_pass(TransformVisitor))
}
