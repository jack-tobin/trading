
from app.rate_limiting import RateLimiting
import dash
from dash import dcc, html, Input, Output, State
import dash_bootstrap_components as dbc
import subprocess
import json
import os
from trading_engine import run_backtest


app = dash.Dash(__name__, external_stylesheets=[dbc.themes.LITERA])
rate_limiting = RateLimiting()

app.layout = html.Div([
    html.H1("Backtesting Engine"),

    html.Div([
        html.Label("Ticker Symbol"),
        dcc.Input(id="ticker", type="text", value="AAPL"),

        dcc.Dropdown(
            id="strategy",
            options=[
                {"label": "MA Crossover", "value": "ma_crossover"},
                # Add more strategies as needed
            ],
            value="ma_crossover",
        ),

        html.Label("Initial Capital"),
        dcc.Input(id="capital", type="number", value=1000000),

        html.Label("Window Size"),
        dcc.Input(id="window", type="number", value=90),

        html.Label("Long Quantity"),
        dcc.Input(id="long-qty", type="number", value=100),

        html.Label("Short Quantity"),
        dcc.Input(id="short-qty", type="number", value=-100),

        html.Button("Run Backtest", id="run-button"),
    ]),

    html.Div(id="results-container"),
])

@app.callback(
    Output("results-container", "children"),
    Input("run-button", "n_clicks"),
    State("strategy", "value"),
    State("ticker", "value"),
    State("window", "value"),
    State("capital", "value"),
    State("long-qty", "value"),
    State("short-qty", "value"),
    prevent_initial_call=True
)
@rate_limiting.limit_backtests(max_requests=10, period=3600)
def run_backtest_callback(
    n_clicks,
    strategy,
    ticker,
    window,
    capital,
    long_qty,
    short_qty,
):
    try:
        backtest_result = run_backtest(strategy, ticker, window, capital, long_qty, short_qty)

        # Extract trades from the portfolio
        trades = backtest_result.get('portfolio', {}).get('trades', [])

        trades_table = html.Table([
            html.Thead(
                html.Tr([
                    html.Th("Date"),
                    html.Th("Type"),
                    html.Th("Symbol"),
                    html.Th("Quantity"),
                    html.Th("Price"),
                    html.Th("Value"),
                ]),
            ),
            html.Tbody([
                html.Tr([
                    html.Td(trade.get('date', '')),
                    html.Td(trade.get('type', '')),
                    html.Td(trade.get('symbol', '')),
                    html.Td(trade.get('quantity', '')),
                    html.Td(f"${trade.get('price', 0):.2f}"),
                    html.Td(f"${trade.get('value', 0):.2f}")
                ]) for trade in trades
            ]),
        ], style={'width': '100%', 'border-collapse': 'collapse'})

        # Display the results
        return html.Div([
            html.H3("Backtest Results"),
            html.P(f"Number of Trades: {backtest_result['n_trades']}"),
            html.H4("Trades"),
            trades_table if trades else html.P("No trades were executed in this backtest.")
        ])
    except subprocess.CalledProcessError as e:
        return html.Div([
            html.H3("Error"),
            html.P(f"Backtest failed: {e.stderr}"),
        ])


def run_server():
    app.run(debug=True, host='0.0.0.0', port=8050)


if __name__ == '__main__':
    run_server()
