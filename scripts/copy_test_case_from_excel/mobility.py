import pandas as pd
from .utils import (
    find_and_replace_arguments,
    convert_values,
    insert_in_section,
    declare_year_value,
    sector_values_padding,
)

def write_test_case_mobility(inputs, measures):

    with open("src/mobility/tests/mobility_test_case.rs", "r") as f:
        lines = f.readlines()

    lines = set_input_variables(inputs, lines)
    lines = set_measures(measures, lines)

    with open("src/mobility/tests/mobility_test_case.rs", "w") as f:
        for line in lines:
            f.write(line)



def set_input_variables(inputs, lines):

    os = -2

    # EW bzw. MA bzw. SuS (in 1.000)
    lines = find_and_replace_arguments(lines, "n_inhabitants__k__", str((convert_values(inputs.iloc[5+os, 1:5]))))
    # Fahrleistung/Pkw (in 1.000 km)
    lines = find_and_replace_arguments(lines, "traveld_dist_per_person__m__m_per_a", str((float(inputs.iloc[23+os, 9]))))
    # Besetzungsquote Pkw privat
    lines = find_and_replace_arguments(lines, "mean_persons_per_car", str((float(inputs.iloc[27+os, 1]))))
    # %Pkw
    lines = find_and_replace_arguments(lines, "modal_split_car", str((float(inputs.iloc[27+os, 9]))))
    # Anzahl Pkw (in 1.000)
    lines = find_and_replace_arguments(lines, "n_cars__k__", convert_values(inputs.iloc[24+os, 1:4]))
    #   davon Anzahl Pkw BEV (in 1.000)
    lines = find_and_replace_arguments(lines, "n_bev__k__", convert_values(inputs.iloc[25+os, 1:4]))
    # Anzahl Laternen (in 1.000)
    lines = find_and_replace_arguments(lines, "n_sl__k__", str((float(inputs.iloc[30+os, 3]))))
    # Stromverbrauch je Laterne (in kWh/a)
    lines = find_and_replace_arguments(lines, "nrg_cnsmp_per_sl__k__W_h_per_a", str((float(inputs.iloc[31+os, 3]))))
    # Andere Kosten je Laterne (in €/a)
    lines = find_and_replace_arguments(lines, "om_costs_per_sl__eur_per_a", str((float(inputs.iloc[32+os, 3]))))

    return lines


def set_measures(measures, lines):

    def measure_line(measures, row, varname, sector):
        return f'\t{"//" if measures.iloc[row-2, 13] == 0 else ""}mobility.inputs.{varname}.{sector}.add_measure("{varname}", {measures.iloc[row-2, 11]}, {measures.iloc[row-2, 12]}, {measures.iloc[row-2, 9] - measures.iloc[row-2, 10]});\n'

    measures_input = []
    # Set Measures
    measures_input.append("\n")
    measures_input.append("\t//Private\n")
    measures_input.append(measure_line(measures, 18, "n_cars__k__", "private"))
    measures_input.append(measure_line(measures, 19, "n_bev__k__", "private"))
    measures_input.append("\n")

    measures_input.append("\t//Industry\n")
    measures_input.append(measure_line(measures, 28, "n_cars__k__", "industry"))
    measures_input.append(measure_line(measures, 29, "n_bev__k__", "industry"))
    measures_input.append("\n")

    measures_input.append("\t//Schools\n")
    measures_input.append("\n")

    measures_input.append("\t//Public\n")
    measures_input.append(measure_line(measures, 46, "n_cars__k__", "public"))
    measures_input.append(measure_line(measures, 47, "n_bev__k__", "public"))
    measures_input.append("\n")

    lines = insert_in_section(lines, measures_input, "[start:measures]", "[end:measures]")

    return lines

def write_excel_comparison_mobility(results):

    with open("src/mobility/tests/compare_with_excel.rs") as f:
        lines = f.readlines()

    lines = declare_variables(results, lines)
    lines = write_assert_statements(results, lines)

    with open("src/mobility/tests/compare_with_excel.rs", "w") as f:
        for line in lines:
            f.write(line)


def declare_variables(results, lines):
    """
    This sets varibales like the price for electric energy, which is not calculated
    in the buildings case and therefore need to be set from outside.
    """

    content = []

    # set power demand buildings except heat pumps
    content = declare_year_value(
        content,
        results,
        'nrg_own_mix_price__m__eur_per_W_h',
        349,
        350,
        2,
        -4,
    )
    content.append("\n")
    content.append("\n")


    # set power consumed by street lights
    # this is ignored right now as it is missing in excel

    lines = insert_in_section(lines, content, "[start:declare_variables]", "[end:declare_variables]")

    return lines


def write_assert_statements(results, lines, years=[2022,2023,2024,2025]):
    """
    This adds the assert statements for the output variables.
    """

    assert_lines = ["\n"]

    for variable, i, param_type, n_sectors in [
        ["n_cars__k__", 186, "inputs", 3],
        ["n_bev__k__", 190, "inputs", 3],
        ["traveld_dist_car__M__m_per_a", 194, "results", 3],
        ["cars_grant__M__eur_per_a", 199, "results", 3],
        ["bev_elec_nrg_dmd__G__W_h_per_a", 207, "results", 3],
        ["cars_fuel_dmd__M__L_per_a", 211, "results", 3],
        ["bev_elec_nrg_price__G__W_h_per_a", 215, "results", 3],
        ["cars_fuel_costs__M__eur_per_a", 219, "results", 3],
        ["cars_ems__k__to_coe_per_a", 366, "results", 3],
    ]:

        name = str(results.iloc[i,0]).replace('\n',' ')
        assert_lines.append(f"\t// {name}\n")
        for j, year in enumerate(years):
            sector_values = ", ".join(
                sector_values_padding(
                    [str(val if not "nan" in str(val) else 0.0) for val in results.iloc[i:i+n_sectors,j+2].values]
                )
            )
            assert_lines.append("\tassert(\n")
            assert_lines.append(f"\tmobility.{param_type}.{variable}.get_year_values({year}),\n")
            assert_lines.append(f"\t\t[{sector_values}],\n")
            assert_lines.append("\t);\n")
        assert_lines.append("\n")


    for variable, i, param_type in [
        ["n_sl__k__", 223, "inputs"],
        ["nrg_cnsmp_per_sl__k__W_h_per_a", 224, "inputs"],
        ["om_costs_per_sl__eur_per_a", 225, "inputs"],
        ["sl_nrg_costs__M__eur_per_a", 226, "results"],
        ["sl_om_costs__M__eur_per_a", 227, "results"],
        ["sl_total_costs__M__eur_per_a", 228, "results"],
    ]:

        name = str(results.iloc[i,0]).replace('\n',' ')
        assert_lines.append(f"\t// {name}\n")
        for j, year in enumerate(years):
            value = str(float(results.iloc[i,j+2]))
            assert_lines.append(f"\tassert_relative_eq!(\n")
            assert_lines.append(f"\t\tmobility.{param_type}.{variable}.get_year({year}),\n")
            assert_lines.append(f"\t\t{value},\n")
            assert_lines.append(f"\t\tmax_relative=0.1,\n")
            assert_lines.append("\t);\n")
        assert_lines.append("\n")

    lines = insert_in_section(lines, assert_lines, "[start:assert_measures]", "[end:assert_measures]")

    return lines
