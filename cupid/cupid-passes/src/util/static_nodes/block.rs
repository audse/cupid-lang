crate::util::node_builder! {
    #[derive(Debug, Default, Clone)]
    pub BlockBuilder => pub Block<E: Default + Clone> {
        pub expressions: Vec<E>,
    }
}

impl<T: Default + Clone> Block<T> {
    pub fn pass<R: Default + Clone>(
        self, 
        fun: impl FnOnce(Vec<T>, &mut crate::Env) -> crate::PassResult<Vec<R>>, 
        env: &mut crate::Env
    ) -> crate::PassResult<Block<R>> {
        let Block { expressions, attr } = self;
        Ok(Block::build()
            .expressions(fun(expressions, env)?)
            .attr(attr)
            .build())
    }
}