use super::Token;

macro_rules! StartLoop {
	($cell: expr, $scope: expr) => {
		"while (c{$cell}s{$scope} > 0) {"
	};
}

pub fn compile<'a>(TokenTree: Vec<Token>) -> Vec<&'a str> {
    use Token::*;
	let mut CurrentSubject: Token;
	let mut CurrentCell: u8 = 0;
	let mut ScopeNum: u8 = 0;
    let mut towrite: Vec<&str> = Vec::new();
    for (i, token) in TokenTree.iter().enumerate() {
        match token {
			Ref => {
				CurrentSubject = Ref;
			}
			Val => {
				CurrentSubject = Val;
			}
			StartLoop => {
				// Until cell - 1 == 0
				towrite.push(StartLoop!(i - 1, ScopeNum))
			}
			EndLoop => {
				towrite.push("}");
			}
			OpenScope => {
				// Creates a new scope.
				ScopeNum += 1;
				
			}
		}
    }
    return towrite;
}