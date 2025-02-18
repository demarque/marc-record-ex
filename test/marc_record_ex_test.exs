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

  test "record 1: has a leader with all the field with the right value", state do
    leader_map =
      state[:records]
      |> Enum.at(0)
      |> Map.get("leader")

    keys = %{
      "record_length" => 1649,
      "status" => "corrected",
      "record_type" => "language_material",
      "bibliographical_level" => "monograph",
      "control_type" => "unspecified",
      "coding_scheme" => "marc8",
      "data_base_address" => 385,
      "encoding_level" => "partial",
      "descriptive_cataloging_form" => "aacr2",
      "multipart_resource_record_level" => "not_applicable"
    }

    assert Enum.all?(leader_map, fn entry ->
             key = elem(entry, 0)
             value = elem(entry, 1)
             elem(Map.fetch(keys, key), 1) == value
           end)
  end
end
