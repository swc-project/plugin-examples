use swc_core::common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    utils::quote_ident,
    visit::{as_folder, noop_visit_mut_type, FoldWith, VisitMut, VisitMutWith},
};
use swc_core::plugin::errors::HANDLER;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

const CONSOLE: &str = "console";
const DEBUG: &str = "debug";
const LOG: &str = "log";
const INFO: &str = "info";

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    noop_visit_mut_type!();
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
    fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
        call_expr.visit_mut_children_with(self);

        if let Callee::Expr(callee) = &mut call_expr.callee {
            if let Expr::Member(member) = &**callee {
                if let (Expr::Ident(obj), MemberProp::Ident(prop)) = (&*member.obj, &member.prop) {
                    if &obj.sym == CONSOLE && &prop.sym == LOG {
                        *callee = Box::new(Expr::Member(MemberExpr {
                            span: DUMMY_SP,
                            obj: member.obj.to_owned(),
                            prop: MemberProp::Ident(quote_ident!(DEBUG)),
                        }));
                    } else if &obj.sym == CONSOLE && &prop.sym == INFO {
                        HANDLER.with(|handler| {
                            handler.struct_span_err(prop.span, &format!("foo")).emit();
                        });
                    }
                }
            }
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}
