//Esta função retorna um Option contendo um f64
//Ela encapsula a lógica de que a divisão pode falhar.
fn dividir(numerador: f64, denominador: f64) -> Option<f64> {
	if denominador == 0.0 {
		None //Divisão por zero! Retornamos  ausência de valor 
	} else {
		Some(numerador / denominador) //Sucesso! Retornamos o resultado dentro de um Some.
	}
}

fn main() {
	let resultado_sucesso = dividir(10.0, 2.0);
	let resultado_falha = dividir(10.0, 0.0);

	//Usamos 'match' para tratar com segurança o retorno da função.
	// O compilador nos força a tratar tanto Some quando None.

	println!("Tratando o caso com sucesso:");
	match resultado_sucesso {
		Some(valor) => println!("O resultado é {}", valor), //Se for Some, extraímos o valor.
		None => println!("Não foi possivel realizar a divisão."), //Se for None, mostra uma mensagem.  
	}

	println!("\nTratando o caso de falha:");
	match resultado_falha {
		Some(valor) => println!("O resultado é {}", valor),
		None => println!("Não foi possivel realizar a divisão."),

	}
}