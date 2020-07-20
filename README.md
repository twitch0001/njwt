# Njwt
WARNING: Not being maintained as I am now going to write an Elixir impl

A JWT validator for a project I am currently working on 

Why a NIF? you may ask. Well I did not want to make my own native elixir impl and it's a good time to get a bit better at rust


## Installation
Add the following to your `mix.exs` file. I do not plan on adding this to hex.pm

```elixir
def deps do
  [
    {:njwt, git: "https://github.com/twitch0001/njwt.git"}
  ]
end
