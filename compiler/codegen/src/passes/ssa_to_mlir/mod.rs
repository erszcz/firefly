mod builder;

use self::builder::*;

use liblumen_diagnostics::CodeMap;
use liblumen_mlir::{self as mlir, Context, OwnedContext};
use liblumen_pass::Pass;
use liblumen_session::Options;
use liblumen_syntax_ssa as syntax_ssa;
use log::debug;

pub struct SsaToMlir<'a> {
    context: Context,
    codemap: &'a CodeMap,
    options: &'a Options,
}
impl<'a> SsaToMlir<'a> {
    pub fn new(context: &OwnedContext, codemap: &'a CodeMap, options: &'a Options) -> Self {
        Self {
            context: **context,
            codemap,
            options,
        }
    }
}

impl<'m> Pass for SsaToMlir<'m> {
    type Input<'a> = syntax_ssa::Module;
    type Output<'a> = Result<mlir::OwnedModule, mlir::OwnedModule>;

    fn run<'a>(&mut self, module: Self::Input<'a>) -> anyhow::Result<Self::Output<'a>> {
        debug!("building mlir module for {}", module.name());

        let builder = ModuleBuilder::new(&module, self.codemap, self.context, self.options);
        builder.build()
    }
}