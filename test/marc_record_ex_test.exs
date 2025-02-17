defmodule MarcRecordExTest do
  use ExUnit.Case
  doctest MarcRecordEx

  test "parser return error for invalid data" do
    content = <<"aa"::binary>>
    {status, _} = MarcRecordEx.parse_records_wrapper(content)
    assert status == :error
  end

  test "parser return a list of all the records" do
    records =
      File.read!("test/fixtures/record.mrc")
      |> MarcRecordEx.parse_records_wrapper()

    assert length(records) == 109
  end
end
