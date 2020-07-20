# Njwt

A simple JWT validator for Elixir.

Why a NIF? you may ask. Well I did not want to make my own native elixir impl and it's a good time to get a bit better at rust


## Installation
Add the following to your `mix.exs` file. I do not plan on adding this to hex.pm

```elixir
def deps do
  [
    {:njwt, git: "https://github.com/twitch0001/njwt.git"}
  ]
end
