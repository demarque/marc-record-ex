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

  test "record 1: there are 5 controls fields ", state do
    controls =
      state[:records]
      |> Enum.at(0)
      |> Map.get("fields")
      |> Enum.filter(fn value -> value["type"] == "control" end)

    assert length(controls) == 5
  end

  test "record 1: there are 25 data fields ", state do
    data =
      state[:records]
      |> Enum.at(0)
      |> Map.get("fields")
      |> Enum.filter(fn value -> value["type"] == "data" end)

    assert length(data) == 25
  end

  test "record 1: control tag 001 has data B301882", state do
    data =
      state[:records]
      |> Enum.at(0)
      |> Map.get("fields")
      |> Enum.filter(fn value -> value["type"] == "control" end)
      |> Enum.find(fn value -> value["tag"] == "001" end)
      |> Map.fetch!("data")

    assert data == "B301882"
  end

  test "record 1: field tag 650 has data indicator 6", state do
    data =
      state[:records]
      |> Enum.at(0)
      |> Map.get("fields")
      |> Enum.filter(fn value -> value["type"] == "data" end)
      |> Enum.find(fn value -> value["tag"] == "650" end)
      |> Map.fetch!("indicator")

    assert data == "6"
  end

  test "record 1: field tag 655 has 2 subfields", state do
    subfields =
      state[:records]
      |> Enum.at(0)
      |> Map.get("fields")
      |> Enum.filter(fn value -> value["type"] == "data" end)
      |> Enum.find(fn value -> value["tag"] == "655" end)
      |> Map.fetch!("subfields")

    assert length(subfields) == 2
  end

  test "record 1: field tag 655 subfields 1 has tag a and value Livres numériques", state do
    subfield =
      state[:records]
      |> Enum.at(0)
      |> Map.get("fields")
      |> Enum.filter(fn value -> value["type"] == "data" end)
      |> Enum.find(fn value -> value["tag"] == "655" end)
      |> Map.fetch!("subfields")
      |> Enum.find(fn value -> value["tag"] == "a" end)

    assert Map.fetch!(subfield, "data") == "Livres numériques."
  end

  test "record 1: field tag 655 subfields 1 has tag 2 and value rvmgf", state do
    subfield =
      state[:records]
      |> Enum.at(0)
      |> Map.get("fields")
      |> Enum.filter(fn value -> value["type"] == "data" end)
      |> Enum.find(fn value -> value["tag"] == "655" end)
      |> Map.fetch!("subfields")
      |> Enum.find(fn value -> value["tag"] == "2" end)

    assert Map.fetch!(subfield, "data") == "rvmgf"
  end
end
