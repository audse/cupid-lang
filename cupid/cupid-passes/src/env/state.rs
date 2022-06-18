use crate::ScopeId;

#[derive(Debug, Clone)]
pub struct EnvState {
    pub(super) current_id: ScopeId,
    pub(super) current_closure: ScopeId,
    pub(super) prev_closures: Vec<ScopeId>,
}

impl Default for EnvState {
    fn default() -> Self {
        Self { current_id: 0, current_closure: 0, prev_closures: vec![0] }
    }
}

impl EnvState {
    pub fn closure(&self) -> ScopeId {
        self.current_closure
    }
    pub fn id(&self) -> ScopeId {
        self.current_id
    }
    pub(super) fn use_closure(&mut self, closure: ScopeId) {
        self.prev_closures.push(self.current_closure);
        self.current_closure = closure;
    }
    pub(super) fn reset_closure(&mut self) {
        let prev_closure = self.prev_closures.pop();
        self.current_closure = prev_closure.unwrap_or_default();
    }
}
