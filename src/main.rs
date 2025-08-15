enum Mensagem {
	Sair,					// Varaiante sem dados 
	Mover { x: i32, y:i32}, //Variante com uma struct anônima
	Escrever(String),		// Variante com uma String 
	MudarCor(i32, i32, i32), //Variante com uma tupla de 3 inteiros 
}

// Enums também podem ter métodos!
impl Mensagem {
	//Este método vai processar a mensagem, dependendo do tipo que ela for.
	fn processar(&self) {
		//2. Usando 'match' para controlar  o fluxo baseado na variante do enum 
		match self{
			Mensagem::Sair => {
				println!("Mensagem: Sair do programa.");
			}
			Mensagem::Mover {x , y} => {
				println!("Movendo cursor para a posição x: {}, y:{}", x, y);
			}
			Mensagem::Escrever(texto) => {
				println!("Escrevendo texto: '{}'", texto);
			}
			Mensagem::MudarCor(r, g, b) =>{
				println!("Mudando a cor para (R:{}, G:{}, B:{})", r, g, b);
			}
			
		}
		
	}
}

fn main() {
	//3. Criando instância das diferentes variantes do nosso enum 
	let msg1 = Mensagem::Escrever(String::from("Olá, Parceiro de Programãção!"));
	let msg2 = Mensagem::Mover {x: 10, y: 20};
	let msg3 = Mensagem::MudarCor(255, 0, 128);
	let msg4 = Mensagem::Sair;

	//4. Chamando o método para processar cada uma delas 
	msg1.processar();
	msg2.processar();
	msg3.processar();
	msg4.processar();
}