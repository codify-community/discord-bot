# Bot do discord oficial da codify community.

# Como eu posso rodar ele?

## Requisitos

- [Compilador do Rust](https://www.rust-lang.org/)
- [Token do discord](https://discord.com/developers/applications)

## Arquivo de configuração

O .env é um arquivo que contém várias váriaveis de âmbiente para configuração. A gente irá usar um para definir o seu token do discord e o id do servidor em que você vai usar o bot.

Um exemplo:

```sh
DISCORD_TOKEN="XXXXXXXXXXXXXXXXXXXXXXXX.XXXXXX.XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
GUILD_ID=12345678
REGISTRO_ID=12345678
LOG_CHANNEL_ID=12345678
DATABASE_LOCATION=banco-de-dados
```

### - DISCORD_TOKEN `field`

> O `DISCORD_TOKEN` é o token do discord, ele é usado para fazer o bot se conectar com o Discord para executar as suas funções, se esse token não for providenciado, o bot irá mandar uma mensagem semelhante à essa:

### - GUILD_ID `field`

O campo `GUILD_ID` é usado para o discord registrar os comandos apenas nesse servidor, se esse id não for providenciado, o bot não terá para onde enviar os comandos, então o bot irá mandar uma mensagem semelhante à essa:

### - LOG_CHANNEL_ID `field`

O campo `LOG_CHANNEL_ID` é responsavel por indicar o bot a onde ele irá enviar as mensagens de log.

### - DATABASE_LOCATION `field`

O campo `LOG_CHANNEL_ID` é responsavel por indicar o bot a onde ele irá salvar os arquivo de banco-de-dados dele.

## Finalmente executando ele

É somente executar: `cargo run --release` que o bot irá iniciar.
