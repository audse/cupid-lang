
#[derive(Debug, Default, Clone)]
pub enum State<A, B, C, D, E, F, G, H, I> {
	PreAnalysis(A),
	PackageResolved(B),
	TypeNamesResolved(C),
	ScopeAnalyzed(D),
	NamesResolved(E),
	TypesInferred(F),
	TypesChecked(G),
	FlowChecked(H),
	Linted(I),
    
    #[default]
    Empty,
}