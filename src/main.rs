struct Usuario {
	ativo: bool,
	nome: String,
	email: String,
	contagem_logins: u64,
}

//Bloco de implementação para a struct usuário
impl Usuario {
	//Este é um método. Ele "pega emprestado" o objeto de forma imutável.
	//'&self' é uma referência à instância da struct na qual o método foi chamado.
	fn descrever(&self) -> String {
		//format! é como println!, mas retorna a String em vez de imprimir.
		format!("{} <{}> - Ativo: {}", self.nome, self.email, self.ativo)
	}
	//Este método "pega emprestado" o objeto de forma mútavel.
	fn desativar (&mut self) {
		self.ativo = false;
	}
}

fn main(){
	//2. Criando uma "instância" (um preenchimento) da nossa struct
	// Note que a instância inteira precisa ser mútavel para alterarmos um campo.
	let mut usuario1 = Usuario {
		email: String::from("allan@exemplo.com"),
		nome: String::from("Allan"),
		ativo:true,
		contagem_logins:1,
	};

	//Chamando nosso método '.descrever()'
	let decricao = usuario1.descrever();
	println!("Descrição inicial: {}", decricao);

	//Chamando nosso método '.desativar()' que modifica o usuário
	usuario1.desativar();

	//Verificando a mudança 
	println!("Descrição final: {}", usuario1.descrever());
}