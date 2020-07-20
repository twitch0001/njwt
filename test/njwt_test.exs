defmodule NjwtTest do
  use ExUnit.Case
  doctest Njwt

  test "greets the world" do
    assert Njwt.hello() == :world
  end
end
