defmodule MarcRecordExTest do
  use ExUnit.Case
  doctest MarcRecordEx

  test "parser return error for invalid data" do
    content = <<"aa"::binary>>
    {status, _} = MarcRecordEx.parse_records_wrapper(content)
    assert status == :error
  end

  test "parser return 109 records from record.mrc" do
    records =
      File.read!("test/fixtures/record.mrc")
      |> MarcRecordEx.parse_records_wrapper()

    assert length(records) == 109
  end

  test "record 1 return 30 fields" do
    fields =
      File.read!("test/fixtures/record.mrc")
      |> MarcRecordEx.parse_records_wrapper()
      |> Enum.at(0)
      |> Map.get("fields")

    assert length(fields) == 30
  end
end
