defmodule MarcRecordEx do
  @moduledoc """
  This module provides a wrapper around the Rust NIF for MARC records.
  """

  @doc """
  Parse a MARC record.

  ## Examples

      iex> MarcRecordEx.parse_records("record.mrc")
      ....

  """
  use Rustler, otp_app: :marc_record_ex, crate: :marc_record_nif

  def parse_records_wrapper(_binary_data), do: error()

  defp error(), do: :erlang.nif_error(:nif_not_loaded)

  def parse_records do
    data = File.read!("record.mrc")
    MarcRecordEx.parse_records_wrapper(data)
  end
end
