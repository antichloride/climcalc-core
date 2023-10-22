from .utils import insert_in_section


def write_excel_comparison_economy(results):

    with open("src/economy/tests/compare_with_excel.rs") as f:
        lines = f.readlines()

    lines = write_assert_statements(results, lines)
    lines = write_assert_statements_stakeholders(results, lines)

    with open("src/economy/tests/compare_with_excel.rs", "w") as f:
        for line in lines:
            f.write(line)


def write_assert_statements(results, lines, years=[2023,2024,2025]):

    assert_lines = ["\n"]

    os = -2
    for variable, i in [
        ["invest_heating_material", 380+os],
        ["invest_heating_work", 381+os],
        ["turnover_heating_crafting_local", 382+os],
        ["turnover_heating_crafting_national", 383+os],
        ["turnover_heating_production_national", 384+os],
        ["invest_heat_demand_material", 385+os],
        ["invest_heat_demand_work", 386+os],
        ["turnover_heat_demand_crafting_local", 387+os],
        ["turnover_heat_demand_crafting_national", 388+os],
        ["turnover_heat_demand_production_national", 389+os],
        ["invest_solar_roof_material", 390+os],
        ["invest_solar_roof_work", 391+os],
        ["maintenance_solar_roof_work", 392+os],
        ["turnover_solar_roof_crafting_local", 393+os],
        ["turnover_solar_roof_crafting_national", 394+os],
        ["invest_solar_landscape_material", 396+os],
        ["invest_solar_landscape_work", 397+os],
        ["maintenance_solar_landscape_work", 398+os],
        ["turnover_solar_landscape_crafting_local", 399+os],
        ["turnover_solar_landscape_crafting_national", 400+os],
        ["turnover_solar_landscape_production_national", 401+os],
        ["n_jobs_crafting_local", 402+os],
        ["n_jobs_crafting_national", 403+os],
        ["n_jobs_production_national", 404+os],
        ["income_local", 407+os],
        ["income_national", 408+os],
        ["income_tax_local", 409+os],
        ["income_tax_national", 410+os],
        ["turnover_local", 411+os],
        ["revenue_local", 412+os],
        ["turnover_national", 413+os],
        ["turnover_tax_national", 414+os],
        ["turnover_tax_local", 415+os],
        ["business_tax_local", 416+os],
        ["business_tax_national", 417+os],
        ["corporate_tax_national", 418+os],
        ["energy_tax_national", 423+os],
    ]:

        name = str(results.iloc[i,1]).replace('\n',' ')
        assert_lines.append(f"\t// {name}\n")
        for j, year in enumerate(years):
            value = str(float(results.iloc[i,j+3]))
            assert_lines.append(f"\tassert_relative_eq!(\n")
            assert_lines.append(f"\t\teconomy.{variable}.get_year({year}),\n")
            assert_lines.append(f"\t\t{value},\n")
            assert_lines.append(f"\t\tmax_relative=1e-3,\n")
            assert_lines.append("\t);\n")
        assert_lines.append("\n")

    lines = insert_in_section(lines, assert_lines, "[start:assert_measures]", "[end:assert_measures]")

    return lines


def write_assert_statements_stakeholders(results, lines, years=[2023,2024,2025]):

    assert_lines = ["\n"]

    os = -2
    for variable, i in [
        ["private_invest", 426+os],
        ["private_effect_of_measures", 427+os],
        ["private_cash_flow_netto", 428+os],
        ["industry_invest", 430+os],
        ["industry_effect_of_measures", 431+os],
        ["industry_profit_from_measures", 432+os],
        ["industry_cash_flow_netto", 433+os],
        ["community_invest", 435+os],
        ["community_effect_of_measures", 436+os],
        ["community_tax_income_from_measures", 437+os],
        ["community_cash_flow_netto", 438+os],
        ["federal_additional_expenses", 440+os],
        ["federal_additional_tax_income", 441+os],
        ["federal_cash_flow_netto", 442+os],
    ]:

        name = str(results.iloc[i,1]).replace('\n',' ')
        assert_lines.append(f"\t// {name}\n")
        for j, year in enumerate(years):
            value = str(float(results.iloc[i,j+3]))
            assert_lines.append(f"\tassert_relative_eq!(\n")
            assert_lines.append(f"\t\tstakeholders.{variable}.get_year({year}),\n")
            assert_lines.append(f"\t\t{value},\n")
            assert_lines.append(f"\t\tmax_relative=1e-3,\n")
            assert_lines.append("\t);\n")
        assert_lines.append("\n")

    lines = insert_in_section(lines, assert_lines, "[start:assert_measures_stakeholders]", "[end:assert_measures_stakeholders]")

    return lines
