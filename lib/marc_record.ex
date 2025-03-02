defmodule MarcRecord do
  @moduledoc """
  This module provides a wrapper around the Rust NIF for MARC records.
  """

  @doc """
  Parse a MARC record.
  """
  use Rustler, otp_app: :marc_record_ex, crate: :marc_record_nif

  def parse_records_wrapper(_binary_data), do: error()

  defp error(), do: :erlang.nif_error(:nif_not_loaded)

  def parse_records(filename) do
    data = File.read!(filename)
    MarcRecord.parse_records_wrapper(data)
  end

  def example do
    parse_records("./samples/marc8_multiple.mrc")
  end
end
