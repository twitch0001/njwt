defmodule Njwt do
  use Rustler, 
    otp_app: :njwt, 
    crate: :njwt,
    target: "x86_64-unknown-linux-gnu"

  @moduledoc """
  Documentation for `Njwt`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Njwt.hello()
      :world

  """
  def hello do
    :world
  end

  def peek_payload(_token), do: :erlang.nif_error(:nif_not_loaded)
  def validate_token(_token, _key), do: :erlang.nif_error(:nif_not_loaded)
end
