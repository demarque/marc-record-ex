defmodule MarcRecordExTest do
  use ExUnit.Case
  doctest MarcRecordEx

  test "parser return error for invalid data" do
    content = <<"aa"::binary>>
    {status, _} = MarcRecordEx.parse_records_wrapper(content)
    assert status == :error
  end
end
