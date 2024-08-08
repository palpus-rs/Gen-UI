use std::collections::{HashMap, HashSet};

use gen_parser::{For, PropsKey, Value};
use gen_utils::common::{fs, Source, Ulid};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;

use crate::ToToken;

use super::{live_design::LiveDesign, role::Role, safe_widget::SafeWidget};

pub trait AutoBuiltinCompile {
    /// widget -> safe_widget (if role is for or if_else) -> insert into AUTO_BUILTIN_WIDGETS -> AutoBuiltinWidgets -> compile
    fn compile<P>(&self, path: P) -> Option<Vec<String>>
    where
        P: AsRef<std::path::Path>;
}

impl AutoBuiltinCompile for Vec<SafeWidget> {
    fn compile<P>(&self, path: P) -> Option<Vec<String>>
    where
        P: AsRef<std::path::Path>,
    {
        if self.is_empty() {
            return None;
        }
        let mut registers = vec![];
        for widget in self {
            match &widget.role {
                Role::For {
                    id,
                    credential,
                    loop_type,
                    props,
                } => {
                    let (source, live_design) =
                        for_widget_to_live_design(widget, id, credential, loop_type, props);

                    // insert target mod into auto/mod.rs
                    fs::append(
                        path.as_ref(),
                        &format!(
                            "pub mod {}; ",
                            source.compiled_file.file_stem().unwrap().to_str().unwrap()
                        ),
                    )
                    .expect("insert auto builtin widget mod failed");
                    registers.push(format!(
                        "crate::{}::live_design(cx);",
                        source.to_live_register()
                    ));
                    // now should compile to source file
                    let _ = fs::write(
                        source.compiled_file.as_path(),
                        &live_design.to_token_stream().to_string(),
                    )
                    .expect("write auto builtin widget source file failed");
                }
                Role::If { .. } => {
                    todo!("if_else widget compile");
                }
                _ => {}
            }
        }
        Some(registers)
    }
}

fn for_widget_to_live_design(
    widget: &SafeWidget,
    ulid: &Ulid,
    credential: &For,
    loop_type: &str,
    props: &HashMap<PropsKey, Value>,
) -> (Source, LiveDesign) {
    let mut live_design = LiveDesign::default();
    // get widget source and change compiled_file to xxx/src_gen/src/auto/${source}.rs ---------------------------------------------------------------
    let mut source = widget.source.as_ref().unwrap().clone();
    source.compiled_file = source
        .compiled_dir
        .as_path()
        .join("src")
        .join("auto")
        .join(&format!("{}_{}.rs", &widget.name, ulid));
    // check current widget is define or is static ---------------------------------------------------------------------------------------------------
    if widget.is_static {
        let widget_name = parse_str::<TokenStream>(&format!("{}{}", &widget.name, ulid)).unwrap();
        let inner_tree = widget.tree.as_ref();
        // generate widget tree code -----------------------------------------------------------------------------------------------------------------
        let tree = quote! {
            #widget_name = {{#widget_name}}{
                item: #inner_tree
            }
        };
        live_design.tree = Some(tree);
        // generate widget logic ---------------------------------------------------------------------------------------------------------------------
        let loop_ident = parse_str::<TokenStream>(&credential.iter_ident).unwrap();
        let loop_type = parse_str::<TokenStream>(&loop_type).unwrap();
        let widget_ref = parse_str::<TokenStream>(&format!("{}{}Ref", &widget.name, ulid)).unwrap();
        let origin_ref = parse_str::<TokenStream>(&format!("{}Ref", &widget.name)).unwrap();
        let set_loop =
            parse_str::<TokenStream>(&format!("set_{}", &loop_ident.to_string())).unwrap();
        let set_widget_props = if props.is_empty() {
            None
        } else {
            let mut set_props = TokenStream::new();

            for (key, value) in props.iter() {
                let set_key = parse_str::<TokenStream>(&format!(
                    "set_{}({})",
                    key.name(),
                    &value.to_string()
                ))
                .unwrap();

                set_props.extend(quote! {
                    target.#set_key;
                });
            }
            Some(set_props)
        };
        // 注意！这个方法需要处理
        let enumerate = parse_str::<TokenStream>(&credential.fmt_enumerate()).unwrap();

        let logic = quote! {
            #[derive(Live, Widget, LiveHook)]
            pub struct #widget_name {
                #[redraw] #[rust] area: Area,
                #[live] item: Option<LivePtr>,
                #[rust] children: ComponentMap<LiveId, #origin_ref>,
                #[layout] layout: Layout,
                #[walk] walk: Walk,
                #[rust] pub #loop_ident: #loop_type,
            }

            impl Widget for #widget_name {
                fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
                    cx.begin_turtle(walk, self.layout);
                    for #enumerate in self.#loop_ident.iter().enumerate() {
                        let target = self.children.get_or_insert(cx, LiveId(index as u64), |cx|{
                            WidgetRef::new_from_ptr(cx, self.item).as_button()
                        });

                        #set_widget_props
                        target.draw_all(cx, &mut Scope::empty());
                    }
                    cx.end_turtle();
                    self.children.retain_visible();
                    DrawStep::done()
                }
                fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                    self.children.iter().enumerate().for_each(|(_index, (_id, widget_ref))|{
                        widget_ref.handle_event(cx, event, scope);
                    });
                }
            }

            impl #widget_ref {
                pub fn #set_loop(&mut self, looper: #loop_type) {
                    if let Some(mut instance) = self.borrow_mut(){
                        instance.#loop_ident = looper;
                    }
                }
            }
        };
        live_design.logic = Some(logic);
    } else {
        todo!("do define widget, not support now");
    }

    (source, live_design)
}
