# Bot do discord oficial da codify community.

# Como eu posso rodar ele?

## Requisitos
- [Compilador de C](https://www.rust-lang.org/)
- [Token do discord](https://discord.com/developers/applications)

## Arquivo de configuração
> Caso você ignore essa parte, o bot irá criar o arquivo automaticamente e irá setar alguns valores por padrão, que você pode mudar depois

O .env é um arquivo que contém várias váriaveis de âmbiente para configuração. A gente irá usar um para definir o seu token do discord e o id do servidor em que você vai usar o bot.

Um exemplo:
```sh
DISCORD_TOKEN="XXXXXXXXXXXXXXXXXXXXXXXX.XXXXXX.XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
CODIFY_GUILD_ID="001122334455667788"
```

### - DISCORD_TOKEN `field`
> O `DISCORD_TOKEN` é o token do discord, ele é usado para fazer o bot se conectar com o Discord para executar as suas funções, se esse token não for providenciado, o bot irá mandar uma mensagem semelhante à essa:
```
/!\ ERRO: Nenhum token de autenticação foi providenciado no arquivo de configuração/argumentos/variáveis de âmbiente!
```

### - CODIFY_GUILD_ID `field`
O campo `CODIFY_GUILD_ID` é usado para o discord registrar os comandos apenas nesse servidor, se esse id não for providenciado, o bot não terá para onde enviar os comandos, então o bot irá mandar uma mensagem semelhante à essa:
```
/!\ ERRO: Nenhum id de servidor foi providenciado no arquivo de configuração/argumentos/variáveis de âmbiente!
```

## Finalmente executando ele
É somente executar: `cargo run --release` que o bot irá iniciar.
