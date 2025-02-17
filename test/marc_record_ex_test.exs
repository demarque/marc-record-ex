defmodule MarcRecordExTest do
  use ExUnit.Case
  doctest MarcRecordEx

  setup_all do
    records =
      File.read!("test/fixtures/record.mrc")
      |> MarcRecordEx.parse_records_wrapper()

    {:ok, records: records}
  end

  test "parser return error for invalid data" do
    content = <<"aa"::binary>>
    {status, _} = MarcRecordEx.parse_records_wrapper(content)
    assert status == :error
  end

  test "parser return 109 records from record.mrc", state do
    assert length(state[:records]) == 109
  end

  test "record 1 return 30 fields", state do
    fields =
      state[:records]
      |> Enum.at(0)
      |> Map.get("fields")

    assert length(fields) == 30
  end
end
