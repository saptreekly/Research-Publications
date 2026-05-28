# Tidy Tuesday 2026-05-26 - Sustainable Energy for All (SE4ALL)
# Run from repo root:
#   julia research-docs/tidy-tuesday/analysis-2026-05-26.jl

using CSV, DataFrames, Statistics, Plots

const DATA_PATH = joinpath(@__DIR__, "data", "energy_cleaned.csv")
const OUT_DIR = joinpath(@__DIR__, "..", "..", "static", "tidy-tuesday", "2026-05-26")

mkpath(OUT_DIR)

energy = CSV.read(DATA_PATH, DataFrame; missingstring="NA")

function json_number(value)
    ismissing(value) && return "null"
    value isa Number || return "null"
    string(float(value))
end

function export_explore_json(df)
    metrics = [
        ("solar", "Solar", "solar_tfec"),
        ("wind", "Wind", "wind_tfec"),
        ("hydro", "Hydro", "hydro_tfec"),
        ("geothermal", "Geothermal", "geothermal_tfec"),
        ("biomass", "Modern biomass", "biomass_tfec"),
        ("solar_tj", "Solar (terajoules)", "solar_tj"),
    ]

    open(joinpath(OUT_DIR, "explore.json"), "w") do io
        println(io, "{")
        print(io, "\"metrics\":[")
        for (index, (id, label, key)) in enumerate(metrics)
            index > 1 && print(io, ",")
            print(io, "{\"id\":\"$id\",\"label\":\"$label\",\"key\":\"$key\"}")
        end
        println(io, "],")
        println(io, "\"records\":[")
        for (index, row) in enumerate(eachrow(df))
            index > 1 && println(io, ",")
            print(io, "{")
            print(io, "\"c\":$(repr(row.country_name)),")
            print(io, "\"y\":$(row.yr),")
            print(io, "\"solar_tfec\":$(json_number(row.solar_energy_consumption_tfec_pct)),")
            print(io, "\"wind_tfec\":$(json_number(row.wind_energy_consumption_tfec_pct)),")
            print(io, "\"hydro_tfec\":$(json_number(row.hydro_energy_consumption_tfec_pct)),")
            print(io, "\"geothermal_tfec\":$(json_number(row.geothermal_energy_consumption_tfec_pct)),")
            print(io, "\"biomass_tfec\":$(json_number(row.modern_biomass_energy_consumption_tfec_pct)),")
            print(io, "\"solar_tj\":$(json_number(row.solar_energy_consumption_terajoules)),")
            print(io, "\"elec_pct\":$(json_number(row.access_electricity_total_pop_pct)),")
            print(io, "\"renew_tfec\":$(json_number(row.renewable_energy_consumption_tfec_pct))")
            print(io, "}")
        end
        println(io, "\n]}")
    end
end

export_explore_json(energy)

# --- Plot 1: Wind adoption in Denmark, Ireland, Norway (Tidy Tuesday sample framing) ---
windy = filter(row -> row.country_name in ("Denmark", "Ireland", "Norway"), energy)
windy = select(windy, :country_name, :yr, :wind_energy_consumption_tfec_pct)
windy = dropmissing(windy, :wind_energy_consumption_tfec_pct)
sort!(windy, [:country_name, :yr])

palette = Dict(
    "Denmark" => "#009e73",
    "Ireland" => "#56b4e9",
    "Norway" => "#d55e00",
)

p1 = plot(
    xlabel = "Year",
    ylabel = "Wind energy consumption (% of TFEC)",
    title = "Wind power consumed by population and industry",
    legend = :topleft,
    size = (900, 520),
    margin = 5Plots.mm,
)

for country in sort(unique(windy.country_name))
    subset = filter(row -> row.country_name == country, windy)
    plot!(
        p1,
        subset.yr,
        subset.wind_energy_consumption_tfec_pct;
        label = country,
        linewidth = 2,
        color = palette[country],
    )
end

savefig(p1, joinpath(OUT_DIR, "wind-adoption-nordics.png"))

# --- Plot 2: Fastest-growing renewable technology (2000–2020 average annual change) ---
renewables = [
    ("Solar", :solar_energy_consumption_tfec_pct),
    ("Wind", :wind_energy_consumption_tfec_pct),
    ("Hydro", :hydro_energy_consumption_tfec_pct),
    ("Geothermal", :geothermal_energy_consumption_tfec_pct),
    ("Modern biomass", :modern_biomass_energy_consumption_tfec_pct),
]

recent = filter(row -> 2000 <= row.yr <= 2010, energy)

growth = map(renewables) do (label, col)
    series = dropmissing(select(recent, :country_name, :yr, col), col)
    slopes = Float64[]
    for sub in groupby(series, :country_name)
        nrow(sub) < 5 && continue
        xs = Float64.(sub.yr)
        ys = Float64.(sub[!, col])
        all(iszero, ys) && continue
        slope = (ys[end] - ys[1]) / (xs[end] - xs[1])
        push!(slopes, slope)
    end
    isempty(slopes) && return (label, 0.0)
    (label, mean(slopes))
end

sort!(growth, by = x -> x[2], rev = true)
labels = first.(growth)
values = last.(growth)

p2 = bar(
    labels,
    values;
    ylabel = "Mean annual change in TFEC share (pp/year, 2000–2010)",
    title = "Which renewables grew fastest on average?",
    color = "#a855f7",
    legend = false,
    xrotation = 35,
    size = (900, 520),
    margin = 5Plots.mm,
)

savefig(p2, joinpath(OUT_DIR, "renewable-growth-rates.png"))

# --- Plot 3: Lowest solar energy consumption (latest year per country) ---
latest = combine(groupby(energy, :country_name), :yr => maximum => :yr)
latest = innerjoin(energy, latest, on = [:country_name, :yr])
solar = select(latest, :country_name, :solar_energy_consumption_terajoules)
solar = dropmissing(solar, :solar_energy_consumption_terajoules)
solar = filter(row -> row.solar_energy_consumption_terajoules > 0, solar)
lowest = first(sort(solar, :solar_energy_consumption_terajoules), 15)
n = nrow(lowest)

p3 = bar(
    1:n,
    lowest.solar_energy_consumption_terajoules,
    orientation = :h,
    yticks = (1:n, lowest.country_name),
    yflip = true,
    xlabel = "Solar energy consumption (terajoules)",
    title = "Countries with the smallest solar consumption (latest year)",
    color = "#0072b2",
    legend = false,
    size = (900, 560),
    margin = 5Plots.mm,
    left_margin = 8Plots.mm,
)

savefig(p3, joinpath(OUT_DIR, "lowest-solar-consumption.png"))

println("Saved plots and explore.json to ", OUT_DIR)
