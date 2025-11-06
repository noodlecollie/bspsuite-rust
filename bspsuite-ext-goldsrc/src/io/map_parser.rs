use logos::Logos;

// A helpful example of how to define tokens:
// https://logos.maciej.codes/examples/json.html

// When expecting a new entity (including worldspawn),
// declared with '{'.
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"\s+")]
enum BaseContext
{
	#[regex(r"//[^\n]*\n")]
	Comment,

	// Begins EntityContext
	#[token("{")]
	OpenBrace,
}

// When processing properties on an entity.
// Nested brushes are declared with '{'.
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"\s+")]
enum EntityContext
{
	#[regex(r"//[^\n]*\n")]
	Comment,

	// Begins BrushContext
	#[token("{")]
	OpenBrace,

	// Falls back to BaseContext
	#[token("}")]
	CloseBrace,

	// Borrowed from the JSON example. We can tweak this if it turns out not to be quite right.
	//          ( Non-term.     |  ( Escaped   | Unicode       ))
	#[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#, |lex| lex.slice().to_owned())]
	QuotedString(String),
}

// When processing brush faces.
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"\s+")]
enum BrushContext
{
	#[regex(r"//[^\n]*\n")]
	Comment,

	// Falls back to EntityContext
	#[token("}")]
	CloseBrace,

	// Begins Point3DContext
	#[token("(")]
	OpenRoundBracket,

	// Begins VectorContext
	#[token("[")]
	OpenSquareBracket,

	// String covers any other chain of characters that does not
	// open a new context.
	#[regex(r"[A-Za-z0-9_][^\s]*")]
	String,
}

// When processing a 3D vector.
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"\s+")]
enum Point3DContext
{
	// Borrowed from the JSON example.
	#[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
	Number(f64),

	// Falls back to BrushContext.
	#[token(")")]
	CloseRoundBracket,
}

// When processing a vector of items.
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"\s+")]
enum VectorContext
{
	// Borrowed from the JSON example.
	#[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
	Number(f64),

	// String covers any other chain of characters that is not a number.
	#[regex(r"[A-Za-z_][^\s]*")]
	String,

	// Falls back to BrushContext.
	#[token("]")]
	CloseSquareBracket,
}

// TODO: Can't run tests for this until we have a MapBlueprint class.
