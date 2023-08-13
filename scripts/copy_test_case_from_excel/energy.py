import pandas as pd
from .utils import (
    find_and_replace_arguments,
    convert_values,
    insert_in_section,
    declare_year_value,
    sector_values_padding,
)

def write_test_case_energy(inputs, measures):

    with open("src/energy/tests/energy_test_case.rs", "r") as f:
        lines = f.readlines()

    lines = set_input_variables(inputs, lines)
    lines = set_measures(measures, lines)

    with open("src/energy/tests/energy_test_case.rs", "w") as f:
        for line in lines:
            f.write(line)

def set_input_variables(inputs, lines):

    os = -2

    # Dachfläche (1.000 qm)
    lines = find_and_replace_arguments(lines, "rf_A__k__m2", convert_values(inputs.iloc[38+os, 1:4]))
    # % davon PV-tauglich
    lines = find_and_replace_arguments(lines, "sol_rf_suitable_A_part", convert_values(inputs.iloc[39+os, 1:4]))
    # PV lokal auf Dach (MWp)
    lines = find_and_replace_arguments(lines, "sol_rf_installed__M__Wp", convert_values(inputs.iloc[41+os, 1:4]))
    #  % Eigenverbrauch PV auf Dach lokal
    lines = find_and_replace_arguments(lines, "sol_rf_self_cnsmp_part", convert_values(inputs.iloc[43+os, 1:4]))

    # Freifläche belegt mit PV (in Hektar)
    lines = find_and_replace_arguments(lines, "sol_os_installed_A__ha", convert_values(inputs.iloc[48+os, 2:4]))
    #  % Eigenverbrauch PV Freifläche
    lines = find_and_replace_arguments(lines, "sol_os_self_cnsmp_part", convert_values())

    # Menge (GWh)
    lines = find_and_replace_arguments(lines, "prchsd_renewable_nrg__G__W_h_per_a", convert_values(inputs.iloc[52+os, 1:4]))
    # Preis (€/kWh)
    lines = find_and_replace_arguments(lines, "renewable_nrg_price__m__eur_per_W_h", convert_values(inputs.iloc[53+os, 1:4]))
    # Zukauf Strom-Mix (in €/kWh)
    lines = find_and_replace_arguments(lines, "nrg_mix_price__m__eur_per_W_h", convert_values(inputs.iloc[55+os, 1:4]))


    return lines


def set_measures(measures, lines):

    def measure_line(measures, row, varname, sector):
        return f'\t{"//" if measures.iloc[row-2, 13] == 0 else ""}energy.inputs.{varname}.{sector}.add_measure("{varname}", {measures.iloc[row-2, 11]}, {measures.iloc[row-2, 12]}, {measures.iloc[row-2, 9] - measures.iloc[row-2, 10]});\n'

    measures_input = []
    # Set Measures
    measures_input.append("\n")
    measures_input.append("\t//Private\n")
    measures_input.append(measure_line(measures, 21, "sol_rf_installed__M__Wp", "private"))
    measures_input.append("\n")

    measures_input.append("\t//Industry\n")
    measures_input.append(measure_line(measures, 31, "sol_rf_installed__M__Wp", "industry"))
    measures_input.append(measure_line(measures, 32, "sol_os_installed_A__ha", "industry"))
    measures_input.append("\n")

    measures_input.append("\t//Schools\n")
    measures_input.append("\n")

    measures_input.append("\t//Public\n")
    measures_input.append(measure_line(measures, 49, "sol_rf_installed__M__Wp", "public"))
    measures_input.append(measure_line(measures, 50, "sol_os_installed_A__ha", "public"))
    measures_input.append("\n")

    lines = insert_in_section(lines, measures_input, "[start:measures]", "[end:measures]")

    return lines


def write_excel_comparison_energy(results):

    with open("src/energy/tests/compare_with_excel.rs") as f:
        lines = f.readlines()

    lines = declare_variables(results, lines)
    lines = write_assert_statements(results, lines)

    with open("src/energy/tests/compare_with_excel.rs", "w") as f:
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
        'buildings_elec_dmd__G__W_h_per_a',
        37,
        41,
        2,
        -4,
    )
    content.append("\n")
    content.append("\n")

    # set power consumed by heat pumps
    content = declare_year_value(
        content,
        results,
        'cnsmp_elec_heat_pump__G__W_h_per_a',
        92,
        96,
        2,
        -4,
    )
    content.append("\n")
    content.append("\n")

    # set power consumed by bev's
    content = declare_year_value(
        content,
        results,
        'bev_elec_nrg_dmd__G__W_h_per_a',
        207,
        210,
        2,
        -4,
    )
    content.append("\n")
    content.append("\n")

    # set power consumed by street lights
    for year in results.columns[2:-4]:
        value = str(float(results.iloc[223][year])*float(results.iloc[224][year])/1000)

        content.append(f"\tsl_nrg_dmd__G__W_h_per_a.set_year_value({year},{value});\n")


    lines = insert_in_section(lines, content, "[start:declare_variables]", "[end:declare_variables]")

    return lines


def write_assert_statements(results, lines, years=[2022,2023,2024,2025]):
    """
    This adds the assert statements for the output variables.
    """

    assert_lines = ["\n"]

    for variable, i, param_type, n_sectors in [
        ["elec_nrg_dmd__G__W_h_per_a", 233, "results", 3],
        ["sol_rf_potential__M__Wp", 239, "results", 3],
        ["sol_rf_installed__M__Wp", 243, "inputs", 3],
        ["sol_rf_nrg__G__W_h_per_a", 247, "results", 3],
        ["sol_rf_self_cnsmp_part", 251, "inputs", 3],
        ["sol_rf_self_cnsmp__G__W_h_per_a", 255, "results", 3],
        ["sol_rf_invest__M__eur_per_a", 261, "results", 3],
        ["sol_rf_grant__M__eur_per_a", 265, "results", 3],
        ["sol_rf_om__M__eur_per_a", 269, "results", 3],
        ["sol_rf_revenue__M__eur_per_a", 274, "results", 3],
        ["sol_os_installed_A__ha", 287, "inputs", 2],
        ["sol_os_installed__M__Wp", 291, "results", 2],
        ["sol_os_nrg__G__W_h_per_a", 294, "results", 2],
        ["sol_os_invest__M__eur_per_a", 299, "results", 2],
        ["sol_os_grant__M__eur_per_a", 302, "results", 2],
        ["sol_os_om__M__eur_per_a", 305, "results", 2],
        ["sol_os_prod_costs__M__eur_per_a", 308, "results", 2],
        ["sol_os_turnover_buyback__M__eur_per_a", 312, "results", 2],
        ["sol_os_revenue__M__eur_per_a", 315, "results", 2],
        ["prchsd_renewable_nrg__G__W_h_per_a", 319, "inputs", 3],
        ["renewable_nrg_price__m__eur_per_W_h", 323, "inputs", 3],
        ["prchsd_renewable_nrg__M__eur_per_a", 326, "results", 3],
        ["prchsd_nrg_mix__G__W_h_per_a", 331, "results", 3],
        ["prchsd_nrg_mix_costs__M__eur_per_a", 338, "results", 3],
        ["prchsd_nrg_mix_ems__k__to_coe_per_a", 362, "results", 3],
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
            assert_lines.append(f"\t\tenergy.{param_type}.{variable}.get_year_values({year}),\n")
            assert_lines.append(f"\t\t[{sector_values}],\n")
            assert_lines.append("\t);\n")
        assert_lines.append("\n")

    lines = insert_in_section(lines, assert_lines, "[start:assert_measures]", "[end:assert_measures]")

    return lines
