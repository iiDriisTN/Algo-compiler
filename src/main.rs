use logos::Logos;
use std::collections::HashMap;


#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\r]+")]
#[logos(skip(r"//[^\r\n]*", allow_greedy = true))]
#[logos(skip r"/\*([^*]|\*[^/])*\*/")]
enum Token {

    #[token("algorithme")]
    Algorithme,

    #[token("à")]
    A,

    #[token("alors")]
    Alors,

    #[token("autres")]
    Autres,

    #[token("de")]
    De,

    #[token("début")]
    Debut,

    #[token("faire")]
    Faire,

    #[token("fin")]
    Fin,

    #[token("fin_pour")]
    FinPour,

    #[token("fin_selon")]
    FinSelon,

    #[token("fin_si")]
    FinSi,

    #[token("fin_tant_que")]
    FinTantQue,

    #[token("jusqu'à")]
    JusquA,

    #[token("pas")]
    Pas,

    #[token("pour")]
    Pour,

    #[token("répéter")]
    Repeter,

    #[token("retourner")]
    Retourner,

    #[token("selon")]
    Selon,

    #[token("si", priority = 10)]
    Si,

    #[token("sinon", priority = 10)]
    Sinon,

    #[token("tant que")]
    TantQue,

    #[token("fonction")]
    Fonction,

    #[token("procédure")]
    Procedure,


    #[token("booléen")]
    TypeBooleen,

    #[token("caractère")]
    TypeCaractere,

    #[token("chaîne")]
    TypeChaine,

    #[token("entier")]
    TypeEntier,

    #[token("réel")]
    TypeReel,

    #[token("tableau")]
    TypeTableau,

    #[token("enregistrement")]
    TypeEnregistrement,

    #[token("fichier texte")]
    TypeFichierTexte,

    #[token("fichier de")]
    TypeFichier,

    #[token("abs")]
    Abs,

    #[token("aléa")]
    Alea,

    #[token("arrondi")]
    Arrondi,

    #[token("chr")]
    Chr,

    #[token("convch")]
    Convch,

    #[token("effacer")]
    Effacer,

    #[token("ent")]
    Ent,

    #[token("estnum")]
    EstNum,

    #[token("fermer")]
    Fermer,

    #[token("fin_fichier")]
    FinFichier,

    #[token("lire")]
    Lire,

    #[token("lire_ligne")]
    LireLigne,

    #[token("long")]
    Long,

    #[token("majus")]
    Majus,

    #[token("ord")]
    Ord,

    #[token("ouvrir")]
    Ouvrir,

    #[token("pos")]
    Pos,

    #[token("racine_carrée")]
    RacineCarree,

    #[token("sous_chaîne")]
    SousChaine,

    #[token("valeur")]
    Valeur,

    #[token("écrire")]
    Ecrire,

    #[token("écrire_nl")]
    EcrireNl,


    #[token("vrai")]
    Vrai,

    #[token("faux")]
    Faux,


    #[token("+")]
    Plus,

    #[token("-")]
    Moins,

    #[token("*")]
    Mult,

    #[token("/")]
    Division,

    #[token("div")]
    DivEntier,

    #[token("mod")]
    Mod,


    #[token("≤")]
    LessEqual,

    #[token("≥")]
    GreaterEqual,

    #[token("<")]
    LessThan,

    #[token(">")]
    GreaterThan,

    #[token("=")]
    Equal,

    #[token("≠")]
    NotEqual,

    #[token("∈")]
    In,

    #[token("<=")]
    LessEqualAscii,

    #[token(">=")]
    GreaterEqualAscii,

    #[token("!=")]
    NotEqualAscii,

    #[token("<>")]
    NotEqualAsciiAlt,

    #[token("in")]
    InAscii,

    #[token("dans")]
    Dans,


    #[token("et")]
    Et,

    #[token("ou")]
    Ou,

    #[token("ouex")]
    OuExclusif,

    #[token("non")]
    Non,

    #[token("<--")]
    Assign,


    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token(":")]
    Colon,


    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().unwrap())]
    Real(f64),

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    Integer(i64),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
    lex.slice()[1..lex.slice().len() - 1].to_string()})]
    String(String),

    #[regex(r"'([^'\\]|\\.)'", |lex| {
    let value = &lex.slice()[1..lex.slice().len() - 1];
    value.chars().next().unwrap()})]
    Character(char),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),




}
#[derive(Debug, Clone, PartialEq)]
enum Type {
    Integer,
    Real,
    Boolean,
    Character,
    String,
}

#[derive(Debug,Clone)]
enum Expr {
    Integer(i64),
    Real(f64),
    Boolean(bool),
    Character(char),
    String(String),

    Identifier(String),

    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },

    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },

    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug,Clone)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    DivInteger,
    Mod,

    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,

    And,
    Or,
    Xor,
    In,
}

#[derive(Debug,Clone)]
enum UnaryOp {
    Neg,
    Not,
}
#[derive(Debug,Clone)]
enum Stmt {
    Assign { target: Expr, value: Expr },

    Return(Expr),

    Call {
        name: String,
        args: Vec<Expr>,
    },

    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Vec<Stmt>,
    },

    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
}
#[derive(Debug,Clone)]
struct Parameter {
    name: String,
    type_: Type,
}
#[derive(Debug, Clone)]
struct Function {
    name: String,
    parameters: Vec<Parameter>,
    return_type: Type,
    body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
struct Procedure {
    name: String,
    parameters: Vec<Parameter>,
    body: Vec<Stmt>,
}
#[derive(Debug, Clone)]
struct Program {
    name: String,
    functions: Vec<Function>,
    procedures: Vec<Procedure>,
    body: Vec<Stmt>,
}
#[derive(Debug,Clone)]
struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

fn builtin_name(token: &Token) -> Option<&'static str> {
    match token {
        Token::Abs => Some("abs"),
        Token::Alea => Some("aléa"),
        Token::Arrondi => Some("arrondi"),
        Token::Chr => Some("chr"),
        Token::Convch => Some("convch"),
        Token::Effacer => Some("effacer"),
        Token::Ent => Some("ent"),
        Token::EstNum => Some("estnum"),
        Token::Fermer => Some("fermer"),
        Token::FinFichier => Some("fin_fichier"),
        Token::Lire => Some("lire"),
        Token::LireLigne => Some("lire_ligne"),
        Token::Long => Some("long"),
        Token::Majus => Some("majus"),
        Token::Ord => Some("ord"),
        Token::Ouvrir => Some("ouvrir"),
        Token::Pos => Some("pos"),
        Token::RacineCarree => Some("racine_carrée"),
        Token::SousChaine => Some("sous_chaîne"),
        Token::Valeur => Some("valeur"),
        Token::Ecrire => Some("écrire"),
        Token::EcrireNl => Some("écrire_nl"),
        _ => None,
    }
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned();
        self.position += 1;
        token
    }

    fn expect(&mut self, expected: Token) {
        let actual = self.advance();

        if actual != Some(expected.clone()) {
            panic!(
                "Expected {:?}, but found {:?}",
                expected, actual
            );
        }
    }

    fn parse_program(&mut self) -> Program {
        self.expect(Token::Algorithme);

        let name = match self.advance() {
            Some(Token::Identifier(name)) => name,
            other => panic!("Expected algorithm name, found {:?}", other),
        };

        self.expect(Token::Debut);

        let mut functions = Vec::new();
        let mut procedures = Vec::new();
        let body = Vec::new();

        while let Some(token) = self.peek() {
            match token {
                Token::Fonction => {
                    functions.push(self.parse_function());
                }

                Token::Procedure => {
                    procedures.push(self.parse_procedure());
                }

                Token::Fin => {
                    break;
                }

                other => {
                    panic!("Unexpected token in program: {:?}", other);
                }
            }
        }

        self.expect(Token::Fin);

        Program {
            name,
            functions,
            procedures,
            body,
        }
    }

    fn parse_type(&mut self) -> Type {
        match self.advance() {
            Some(Token::TypeEntier) => Type::Integer,
            Some(Token::TypeReel) => Type::Real,
            Some(Token::TypeBooleen) => Type::Boolean,
            Some(Token::TypeCaractere) => Type::Character,
            Some(Token::TypeChaine) => Type::String,

            other => panic!("Expected type, found {:?}", other),
        }
    }

    fn parse_function(&mut self) -> Function {
        self.expect(Token::Fonction);

        let name = match self.advance() {
            Some(Token::Identifier(name)) => name,
            other => panic!("Expected function name, found {:?}", other),
        };

        self.expect(Token::LParen);

        let mut parameters = Vec::new();

        if self.peek() != Some(&Token::RParen) {
            let mut names = Vec::new();

            names.push(match self.advance() {
                Some(Token::Identifier(name)) => name,
                other => panic!("Expected parameter name, found {:?}", other),
            });

            while self.peek() == Some(&Token::Comma) {
                self.advance();

                names.push(match self.advance() {
                    Some(Token::Identifier(name)) => name,
                    other => panic!("Expected parameter name, found {:?}", other),
                });
            }

            self.expect(Token::Colon);

            let type_ = self.parse_type();

            for name in names {
                parameters.push(Parameter {
                    name,
                    type_: type_.clone(),
                });
            }
        }

        self.expect(Token::RParen);

        self.expect(Token::Colon);

        let return_type = self.parse_type();

        self.expect(Token::Debut);

        let mut body = Vec::new();

        while self.peek() != Some(&Token::Fin) {
            body.push(self.parse_statement());
        }

        self.expect(Token::Fin);

        Function {
            name,
            parameters,
            return_type,
            body,
        }
    }
    fn parse_procedure(&mut self) -> Procedure {
        self.expect(Token::Procedure);

        let name = match self.advance() {
            Some(Token::Identifier(name)) => name,
            other => panic!("Expected procedure name, found {:?}", other),
        };

        self.expect(Token::LParen);

        let mut parameters = Vec::new();

        if self.peek() != Some(&Token::RParen) {
            let mut names = Vec::new();

            names.push(match self.advance() {
                Some(Token::Identifier(name)) => name,
                other => panic!("Expected parameter name, found {:?}", other),
            });

            while self.peek() == Some(&Token::Comma) {
                self.advance();

                names.push(match self.advance() {
                    Some(Token::Identifier(name)) => name,
                    other => panic!("Expected parameter name, found {:?}", other),
                });
            }

            self.expect(Token::Colon);

            let type_ = self.parse_type();

            for name in names {
                parameters.push(Parameter {
                    name,
                    type_: type_.clone(),
                });
            }
        }

        self.expect(Token::RParen);

        self.expect(Token::Debut);

        let mut body = Vec::new();

        while self.peek() != Some(&Token::Fin) {
            body.push(self.parse_statement());
        }

        self.expect(Token::Fin);

        Procedure {
            name,
            parameters,
            body,
        }
    }
    fn parse_statement(&mut self) -> Stmt {
        match self.peek() {
            Some(Token::Si) => self.parse_if(),
            Some(Token::TantQue) => self.parse_while(),

            Some(Token::Retourner) => self.parse_return(),

            Some(token) if builtin_name(token).is_some() => {
                self.parse_builtin_call_statement()
            }

            Some(Token::Identifier(_)) => {
                if matches!(
                self.tokens.get(self.position + 1),
                Some(Token::LParen)
            ) {
                    self.parse_call_statement()
                } else {
                    self.parse_assignment()
                }
            }

            other => {
                panic!(
                    "Unexpected token in statement: {:?}",
                    other
                );
            }
        }
    }
    fn parse_assignment(&mut self) -> Stmt {
        let target = match self.advance() {
            Some(Token::Identifier(name)) => Expr::Identifier(name),

            other => {
                panic!("Expected assignment target, found {:?}", other);
            }
        };

        self.expect(Token::Assign);

        let value = self.parse_expression();

        Stmt::Assign {
            target,
            value,
        }
    }
    fn parse_call_statement(&mut self) -> Stmt {
        let name = match self.advance() {
            Some(Token::Identifier(name)) => name,

            other => {
                panic!(
                    "Expected procedure name, found {:?}",
                    other
                );
            }
        };

        self.expect(Token::LParen);

        let mut args = Vec::new();

        if self.peek() != Some(&Token::RParen) {
            args.push(self.parse_expression());

            while self.peek() == Some(&Token::Comma) {
                self.advance();

                args.push(self.parse_expression());
            }
        }

        self.expect(Token::RParen);

        Stmt::Call {
            name,
            args,
        }
    }

    fn parse_return(&mut self) -> Stmt {
        self.expect(Token::Retourner);

        let expr = self.parse_expression();

        Stmt::Return(expr)
    }

    fn parse_if(&mut self) -> Stmt {

        self.expect(Token::Si);

        let condition = self.parse_expression();

        self.expect(Token::Alors);

        let mut then_branch = Vec::new();

        while self.peek() != Some(&Token::FinSi)
            && self.peek() != Some(&Token::Sinon)
        {
            then_branch.push(self.parse_statement());
        }

        let mut else_branch = Vec::new();

        if self.peek() == Some(&Token::Sinon) {
            self.advance();

            if self.peek() == Some(&Token::Si) {
                // sinon si ...
                else_branch.push(self.parse_if_without_fin_si());
            } else {
                while self.peek() != Some(&Token::FinSi) {
                    else_branch.push(self.parse_statement());
                }
            }
        }

        self.expect(Token::FinSi);

        Stmt::If {
            condition,
            then_branch,
            else_branch,
        }
    }
    fn parse_while(&mut self) -> Stmt {
        self.expect(Token::TantQue);

        let condition = self.parse_expression();

        self.expect(Token::Faire);

        let mut body = Vec::new();

        while self.peek() != Some(&Token::FinTantQue) {
            body.push(self.parse_statement());
        }

        self.expect(Token::FinTantQue);

        Stmt::While {
            condition,
            body,
        }
    }
    fn parse_if_without_fin_si(&mut self) -> Stmt {
        self.expect(Token::Si);

        let condition = self.parse_expression();

        self.expect(Token::Alors);

        let mut then_branch = Vec::new();

        while self.peek() != Some(&Token::FinSi)
            && self.peek() != Some(&Token::Sinon)
        {
            then_branch.push(self.parse_statement());
        }

        let mut else_branch = Vec::new();

        if self.peek() == Some(&Token::Sinon) {
            self.advance();

            if self.peek() == Some(&Token::Si) {
                else_branch.push(self.parse_if_without_fin_si());
            } else {
                while self.peek() != Some(&Token::FinSi) {
                    else_branch.push(self.parse_statement());
                }
            }
        }

        Stmt::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Expr {
        let mut left = self.parse_and();

        loop {
            let op = match self.peek() {
                Some(Token::Ou) => BinaryOp::Or,
                Some(Token::OuExclusif) => BinaryOp::Xor,
                _ => break,
            };

            self.advance();

            let right = self.parse_and();

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_and(&mut self) -> Expr {
        let mut left = self.parse_equality();

        while self.peek() == Some(&Token::Et) {
            self.advance();

            let right = self.parse_equality();

            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_equality(&mut self) -> Expr {
        let mut left = self.parse_comparison();

        loop {
            let op = match self.peek() {
                Some(Token::Equal) => BinaryOp::Equal,

                Some(Token::NotEqual)
                | Some(Token::NotEqualAscii)
                | Some(Token::NotEqualAsciiAlt) => BinaryOp::NotEqual,

                _ => break,
            };

            self.advance();

            let right = self.parse_comparison();

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut left = self.parse_term();

        loop {
            let op = match self.peek() {
                Some(Token::LessThan) => BinaryOp::LessThan,

                Some(Token::LessEqual)
                | Some(Token::LessEqualAscii) => BinaryOp::LessEqual,

                Some(Token::GreaterThan) => BinaryOp::GreaterThan,

                Some(Token::GreaterEqual)
                | Some(Token::GreaterEqualAscii) => BinaryOp::GreaterEqual,

                Some(Token::In)
                | Some(Token::InAscii)
                | Some(Token::Dans) => BinaryOp::In,

                _ => break,
            };

            self.advance();

            let right = self.parse_term();

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();

        loop {
            let op = match self.peek() {
                Some(Token::Plus) => BinaryOp::Add,
                Some(Token::Moins) => BinaryOp::Sub,
                _ => break,
            };

            self.advance();

            let right = self.parse_factor();

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_factor(&mut self) -> Expr {
        let mut left = self.parse_unary();

        loop {
            let op = match self.peek() {
                Some(Token::Mult) => BinaryOp::Mul,
                Some(Token::Division) => BinaryOp::Div,
                Some(Token::DivEntier) => BinaryOp::DivInteger,
                Some(Token::Mod) => BinaryOp::Mod,
                _ => break,
            };

            self.advance();

            let right = self.parse_unary();

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_unary(&mut self) -> Expr {
        match self.peek() {
            Some(Token::Moins) => {
                self.advance();

                Expr::Unary {
                    op: UnaryOp::Neg,
                    expr: Box::new(self.parse_unary()),
                }
            }

            Some(Token::Non) => {
                self.advance();

                Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(self.parse_unary()),
                }
            }

            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Expr {
        match self.advance() {
            Some(Token::String(value)) => Expr::String(value),

            Some(Token::Character(value)) => Expr::Character(value),

            Some(Token::Integer(value)) => Expr::Integer(value),

            Some(Token::Real(value)) => Expr::Real(value),

            Some(Token::Vrai) => Expr::Boolean(true),

            Some(Token::Faux) => Expr::Boolean(false),

            Some(token) if builtin_name(&token).is_some() => {
                let name = builtin_name(&token)
                    .unwrap()
                    .to_string();

                self.expect(Token::LParen);

                let mut args = Vec::new();

                if self.peek() != Some(&Token::RParen) {
                    args.push(self.parse_expression());

                    while self.peek() == Some(&Token::Comma) {
                        self.advance();
                        args.push(self.parse_expression());
                    }
                }

                self.expect(Token::RParen);

                Expr::Call {
                    name,
                    args,
                }
            }

            Some(Token::Identifier(name)) => {
                if self.peek() == Some(&Token::LParen) {
                    self.advance();

                    let mut args = Vec::new();

                    if self.peek() != Some(&Token::RParen) {
                        args.push(self.parse_expression());

                        while self.peek() == Some(&Token::Comma) {
                            self.advance();
                            args.push(self.parse_expression());
                        }
                    }

                    self.expect(Token::RParen);

                    Expr::Call {
                        name,
                        args,
                    }
                } else {
                    Expr::Identifier(name)
                }
            }

            Some(Token::LParen) => {
                let expr = self.parse_expression();

                self.expect(Token::RParen);

                expr
            }

            other => {
                panic!("Expected expression, found {:?}", other);
            }
        }
    }
    fn parse_builtin_call_statement(&mut self) -> Stmt {
        let token = self.advance().unwrap();

        let name = builtin_name(&token)
            .unwrap()
            .to_string();

        self.expect(Token::LParen);

        let mut args = Vec::new();

        if self.peek() != Some(&Token::RParen) {
            args.push(self.parse_expression());

            while self.peek() == Some(&Token::Comma) {
                self.advance();
                args.push(self.parse_expression());
            }
        }

        self.expect(Token::RParen);

        Stmt::Call {
            name,
            args,
        }
    }

}
struct SymbolTable {
    variables: HashMap<String, Type>,
}

impl SymbolTable {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    fn add_variable(&mut self, name: String, type_: Type) {
        self.variables.insert(name, type_);
    }

    fn get_variable(&self, name: &str) -> Option<&Type> {
        self.variables.get(name)
    }
}
fn build_callable_table(program: &Program) -> CallableTable {
    let mut table = CallableTable::new();

    for function in &program.functions {
        table.add_function(function);
    }

    for procedure in &program.procedures {
        table.add_procedure(procedure);
    }

    // Built-in functions
    table.callables.insert(
        "abs".to_string(),
        Callable::BuiltinFunction {
            parameters: vec![Type::Integer],
            return_type: Type::Integer,
        },
    );

    table.callables.insert(
        "long".to_string(),
        Callable::BuiltinFunction {
            parameters: vec![Type::String],
            return_type: Type::Integer,
        },
    );

    table.callables.insert(
        "ord".to_string(),
        Callable::BuiltinFunction {
            parameters: vec![Type::Character],
            return_type: Type::Integer,
        },
    );

    table.callables.insert(
        "chr".to_string(),
        Callable::BuiltinFunction {
            parameters: vec![Type::Integer],
            return_type: Type::Character,
        },
    );

    // Built-in procedures
    table.callables.insert(
        "écrire_nl".to_string(),
        Callable::BuiltinProcedure {
            parameters: vec![],
        },
    );

    table
}
#[derive(Debug, Clone)]
enum Callable {
    Function {
        parameters: Vec<Parameter>,
        return_type: Type,
    },

    Procedure {
        parameters: Vec<Parameter>,
    },

    BuiltinFunction {
        parameters: Vec<Type>,
        return_type: Type,
    },

    BuiltinProcedure {
        parameters: Vec<Type>,
    },
}

struct CallableTable {
    callables: HashMap<String, Callable>,
}

impl CallableTable {
    fn new() -> Self {
        Self {
            callables: HashMap::new(),
        }
    }

    fn add_function(&mut self, function: &Function) {
        self.callables.insert(
            function.name.clone(),
            Callable::Function {
                parameters: function.parameters.clone(),
                return_type: function.return_type.clone(),
            },
        );
    }

    fn add_procedure(&mut self, procedure: &Procedure) {
        self.callables.insert(
            procedure.name.clone(),
            Callable::Procedure {
                parameters: procedure.parameters.clone(),
            },
        );
    }

    fn get(&self, name: &str) -> Option<&Callable> {
        self.callables.get(name)
    }
}
fn infer_expr_type(
    expr: &Expr,
    symbols: &SymbolTable,
    callables: &CallableTable,
) -> Result<Type, String> {
    match expr {
        Expr::Integer(_) => Ok(Type::Integer),

        Expr::Real(_) => Ok(Type::Real),

        Expr::Boolean(_) => Ok(Type::Boolean),

        Expr::Character(_) => Ok(Type::Character),

        Expr::String(_) => Ok(Type::String),

        Expr::Identifier(name) => {
            match symbols.get_variable(name) {
                Some(type_) => Ok(type_.clone()),

                None => Err(format!(
                    "Unknown variable '{}'",
                    name
                )),
            }
        }

        Expr::Binary { left, op, right } => {
            let left_type =
                infer_expr_type(left, symbols, callables)?;

            let right_type =
                infer_expr_type(right, symbols, callables)?;

            infer_binary_type(
                op,
                &left_type,
                &right_type,
            )
        }

        Expr::Unary { op, expr } => {
            let expr_type =
                infer_expr_type(expr, symbols, callables)?;

            infer_unary_type(
                op,
                &expr_type,
            )
        }

        Expr::Call { name, args } => {
            let callable = match callables.get(name) {
                Some(callable) => callable,

                None => {
                    return Err(format!(
                        "Unknown function or procedure '{}'",
                        name
                    ));
                }
            };

            match callable {
                Callable::Function {
                    parameters,
                    return_type,
                } => {
                    if args.len() != parameters.len() {
                        return Err(format!(
                            "Function '{}' expects {} argument(s), got {}",
                            name,
                            parameters.len(),
                            args.len()
                        ));
                    }

                    for (i, (arg, parameter)) in
                        args.iter().zip(parameters.iter()).enumerate()
                    {
                        let arg_type =
                            infer_expr_type(
                                arg,
                                symbols,
                                callables,
                            )?;

                        if arg_type != parameter.type_ {
                            return Err(format!(
                                "Invalid argument {} in function '{}': expected {:?}, got {:?}",
                                i + 1,
                                name,
                                parameter.type_,
                                arg_type
                            ));
                        }
                    }

                    Ok(return_type.clone())
                }

                Callable::BuiltinFunction {
                    parameters,
                    return_type,
                } => {
                    if args.len() != parameters.len() {
                        return Err(format!(
                            "Built-in function '{}' expects {} argument(s), got {}",
                            name,
                            parameters.len(),
                            args.len()
                        ));
                    }

                    for (i, (arg, expected_type)) in
                        args.iter().zip(parameters.iter()).enumerate()
                    {
                        let arg_type =
                            infer_expr_type(
                                arg,
                                symbols,
                                callables,
                            )?;

                        if &arg_type != expected_type {
                            return Err(format!(
                                "Invalid argument {} in built-in function '{}': expected {:?}, got {:?}",
                                i + 1,
                                name,
                                expected_type,
                                arg_type
                            ));
                        }
                    }

                    Ok(return_type.clone())
                }

                Callable::Procedure { .. } => {
                    Err(format!(
                        "Procedure '{}' does not return a value",
                        name
                    ))
                }

                Callable::BuiltinProcedure { .. } => {
                    Err(format!(
                        "Built-in procedure '{}' does not return a value",
                        name
                    ))
                }
            }
        }
    }
}

fn infer_binary_type(
    op: &BinaryOp,
    left: &Type,
    right: &Type,
) -> Result<Type, String> {
    match op {
        BinaryOp::Add
        | BinaryOp::Sub
        | BinaryOp::Mul
        | BinaryOp::Div => {
            if left == &Type::Integer && right == &Type::Integer {
                Ok(Type::Integer)
            } else if left == &Type::Real && right == &Type::Real {
                Ok(Type::Real)
            } else {
                Err(format!(
                    "Invalid operands for arithmetic operation: {:?} and {:?}",
                    left, right
                ))
            }
        }

        BinaryOp::DivInteger
        | BinaryOp::Mod => {
            if left == &Type::Integer && right == &Type::Integer {
                Ok(Type::Integer)
            } else {
                Err(format!(
                    "Operation requires two integers, got {:?} and {:?}",
                    left, right
                ))
            }
        }

        BinaryOp::Equal
        | BinaryOp::NotEqual
        | BinaryOp::LessThan
        | BinaryOp::LessEqual
        | BinaryOp::GreaterThan
        | BinaryOp::GreaterEqual => {
            if left == right {
                Ok(Type::Boolean)
            } else {
                Err(format!(
                    "Cannot compare {:?} with {:?}",
                    left, right
                ))
            }
        }

        BinaryOp::And
        | BinaryOp::Or
        | BinaryOp::Xor => {
            if left == &Type::Boolean
                && right == &Type::Boolean
            {
                Ok(Type::Boolean)
            } else {
                Err(format!(
                    "Logical operation requires booleans, got {:?} and {:?}",
                    left, right
                ))
            }
        }

        BinaryOp::In => {
            Ok(Type::Boolean)
        }
    }
}

fn infer_unary_type(
    op: &UnaryOp,
    expr_type: &Type,
) -> Result<Type, String> {
    match op {
        UnaryOp::Neg => {
            match expr_type {
                Type::Integer | Type::Real => Ok(expr_type.clone()),

                _ => Err(format!(
                    "Cannot negate {:?}",
                    expr_type
                )),
            }
        }

        UnaryOp::Not => {
            if expr_type == &Type::Boolean {
                Ok(Type::Boolean)
            } else {
                Err(format!(
                    "Cannot use 'non' on {:?}",
                    expr_type
                ))
            }
        }
    }
}
fn check_statement(
    stmt: &Stmt,
    symbols: &SymbolTable,
    expected_return_type: Option<&Type>,
    callables: &CallableTable,
) -> Result<(), String> {
    match stmt {
        Stmt::Assign { target, value } => {
            let target_name = match target {
                Expr::Identifier(name) => name,

                _ => {
                    return Err(
                        "Assignment target must be a variable"
                            .to_string()
                    );
                }
            };

            let target_type =
                match symbols.get_variable(target_name) {
                    Some(type_) => type_.clone(),

                    None => {
                        return Err(format!(
                            "Unknown variable '{}'",
                            target_name
                        ));
                    }
                };

            let value_type =
                infer_expr_type(
                    value,
                    symbols,
                    callables,
                )?;

            if target_type != value_type {
                return Err(format!(
                    "Cannot assign {:?} to variable '{}' of type {:?}",
                    value_type,
                    target_name,
                    target_type
                ));
            }

            Ok(())
        }

        Stmt::Return(expr) => {
            let expected_type = match expected_return_type {
                Some(type_) => type_,

                None => {
                    return Err(
                        "A procedure cannot return a value"
                            .to_string()
                    );
                }
            };

            let return_type =
                infer_expr_type(
                    expr,
                    symbols,
                    callables,
                )?;

            if &return_type != expected_type {
                return Err(format!(
                    "Invalid return type: expected {:?}, got {:?}",
                    expected_type,
                    return_type
                ));
            }

            Ok(())
        }

        Stmt::Call { name, args } => {
            let callable = match callables.get(name) {
                Some(callable) => callable,

                None => {
                    return Err(format!(
                        "Unknown function or procedure '{}'",
                        name
                    ));
                }
            };

            match callable {
                Callable::Function { .. } => {
                    Err(format!(
                        "Function '{}' cannot be called as a statement",
                        name
                    ))
                }

                Callable::BuiltinFunction { .. } => {
                    Err(format!(
                        "Built-in function '{}' cannot be called as a statement",
                        name
                    ))
                }

                Callable::Procedure { parameters } => {
                    if args.len() != parameters.len() {
                        return Err(format!(
                            "Procedure '{}' expects {} argument(s), got {}",
                            name,
                            parameters.len(),
                            args.len()
                        ));
                    }

                    for (i, (arg, parameter)) in
                        args.iter().zip(parameters.iter()).enumerate()
                    {
                        let arg_type =
                            infer_expr_type(
                                arg,
                                symbols,
                                callables,
                            )?;

                        if arg_type != parameter.type_ {
                            return Err(format!(
                                "Invalid argument {} in procedure '{}': expected {:?}, got {:?}",
                                i + 1,
                                name,
                                parameter.type_,
                                arg_type
                            ));
                        }
                    }

                    Ok(())
                }

                Callable::BuiltinProcedure { parameters } => {
                    if args.len() != parameters.len() {
                        return Err(format!(
                            "Built-in procedure '{}' expects {} argument(s), got {}",
                            name,
                            parameters.len(),
                            args.len()
                        ));
                    }

                    for (i, (arg, expected_type)) in
                        args.iter().zip(parameters.iter()).enumerate()
                    {
                        let arg_type =
                            infer_expr_type(
                                arg,
                                symbols,
                                callables,
                            )?;

                        if &arg_type != expected_type {
                            return Err(format!(
                                "Invalid argument {} in built-in procedure '{}': expected {:?}, got {:?}",
                                i + 1,
                                name,
                                expected_type,
                                arg_type
                            ));
                        }
                    }

                    Ok(())
                }
            }
        }

        Stmt::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let condition_type =
                infer_expr_type(
                    condition,
                    symbols,
                    callables,
                )?;

            if condition_type != Type::Boolean {
                return Err(format!(
                    "If condition must be Boolean, got {:?}",
                    condition_type
                ));
            }

            for stmt in then_branch {
                check_statement(
                    stmt,
                    symbols,
                    expected_return_type,
                    callables,
                )?;
            }

            for stmt in else_branch {
                check_statement(
                    stmt,
                    symbols,
                    expected_return_type,
                    callables,
                )?;
            }

            Ok(())
        }
        Stmt::While {
            condition,
            body,
        } => {
            let condition_type =
                infer_expr_type(
                    condition,
                    symbols,
                    callables,
                )?;

            if condition_type != Type::Boolean {
                return Err(format!(
                    "While condition must be Boolean, got {:?}",
                    condition_type
                ));
            }

            for stmt in body {
                check_statement(
                    stmt,
                    symbols,
                    expected_return_type,
                    callables,
                )?;
            }

            Ok(())
        }

    }
}
fn check_function(
    function: &Function,
    symbols: &SymbolTable,
    callables: &CallableTable,
) -> Result<(), String> {
    let mut local_symbols = SymbolTable {
    variables: symbols.variables.clone(),
};

    for parameter in &function.parameters {
        local_symbols.add_variable(
            parameter.name.clone(),
            parameter.type_.clone(),
        );
    }

    for stmt in &function.body {
        check_statement(
            stmt,
            &local_symbols,
            Some(&function.return_type),
            callables,
        )?;
    }

    Ok(())
}

fn check_procedure(
    procedure: &Procedure,
    symbols: &SymbolTable,
    callables: &CallableTable,
) -> Result<(), String> {
    let mut local_symbols = SymbolTable {
    variables: symbols.variables.clone(),
};

    for parameter in &procedure.parameters {
        local_symbols.add_variable(
            parameter.name.clone(),
            parameter.type_.clone(),
        );
    }

    for stmt in &procedure.body {
        check_statement(
            stmt,
            &local_symbols,
            None,
            callables,
        )?;
    }

    Ok(())
}


// fn main() {
//     let source = r#"
// algorithme test
// début
//     fonction test(n:entier) : entier
//     début
//         n <-- n + 1
//         retourner n
//     fin
// fin
// "#;
//
//     let tokens: Vec<Token> = Token::lexer(source)
//         .map(|result| result.expect("Lexer error"))
//         .collect();
//
//     let mut parser = Parser::new(tokens);
//
//     let program = parser.parse_program();
//
//     println!("{:#?}", program);
// }

// fn main() {
//     let source = r#"
// algorithme test
// début
//
//     fonction addition(n,d:entier) : entier
//     début
//         retourner n + d
//     fin
//
//     procédure afficher(n:entier)
//     début
//         n <-- n + 1
//     fin
//
// fin
// "#;
//     let tokens: Vec<Token> = Token::lexer(source)
//         .map(|result| result.expect("Lexer error"))
//         .collect();
//
//     let mut parser = Parser::new(tokens);
//
//     let program = parser.parse_program();
//
//     println!("{:#?}", program);
//
//     for function in &program.functions {
//         match check_function(function, &SymbolTable::new()) {
//             Ok(()) => {
//                 println!(
//                     "Function '{}' is valid.",
//                     function.name
//                 );
//             }
//
//             Err(error) => {
//                 println!(
//                     "Semantic error in '{}': {}",
//                     function.name,
//                     error
//                 );
//             }
//         }
//     }
//     for procedure in &program.procedures {
//         match check_procedure(procedure, &SymbolTable::new()) {
//             Ok(()) => {
//                 println!(
//                     "Procedure '{}' is valid.",
//                     procedure.name
//                 );
//             }
//
//             Err(error) => {
//                 println!(
//                     "Semantic error in procedure '{}': {}",
//                     procedure.name,
//                     error
//                 );
//             }
//         }
//     }
// }


fn main() {
    let source = r#"
algorithme test
début

    fonction test(n:entier) : entier
    début

        x <-- n

        tant que n > 0 faire
            n <-- n - 1
        fin_tant_que

        retourner n

    fin

fin
"#;

    // =========================
    // LEXING
    // =========================

    let lexer = Token::lexer(source);

    let mut tokens = Vec::new();

    for result in lexer {
        match result {
            Ok(token) => tokens.push(token),
            Err(_) => {
                eprintln!("Lexer error");
                return;
            }
        }
    }

    // =========================
    // PARSING
    // =========================

    let mut parser = Parser::new(tokens);

    let program = parser.parse_program();

    println!("{:#?}", program);

    // =========================
    // CALLABLE TABLE
    // =========================

    let callable_table = build_callable_table(&program);

    println!("\nCallable table:");
    println!("{:#?}", callable_table.callables);

    // =========================
    // EXTERNAL VARIABLES
    // =========================

    let mut symbols = SymbolTable::new();

    // These represent variables coming
    // from the external declaration table / IDE.
    symbols.add_variable("x".to_string(), Type::Integer);
    symbols.add_variable("y".to_string(), Type::Real);

    // =========================
    // SEMANTIC CHECKING
    // =========================

    for function in &program.functions {
        match check_function(
            function,
            &symbols,
            &callable_table,
        ) {
            Ok(_) => {
                println!(
                    "Function '{}' is valid.",
                    function.name
                );
            }

            Err(error) => {
                println!(
                    "Semantic error in '{}': {}",
                    function.name,
                    error
                );
            }
        }
    }

    for procedure in &program.procedures {
        match check_procedure(
            procedure,
            &symbols,
            &callable_table,
        ) {
            Ok(_) => {
                println!(
                    "Procedure '{}' is valid.",
                    procedure.name
                );
            }

            Err(error) => {
                println!(
                    "Semantic error in '{}': {}",
                    procedure.name,
                    error
                );
            }
        }
    }
}
