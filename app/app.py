
from rate_limiting import RateLimiting
import dash
from dash import dcc, html, Input, Output, State
import subprocess
import json
import os

app = dash.Dash(__name__)
rate_limiting = RateLimiting()

app.layout = html.Div([
    html.H1("Rust Backtesting Engine"),

    html.Div([
        html.Label("Ticker Symbol"),
        dcc.Input(id="ticker", type="text", value="AAPL"),

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
    State("ticker", "value"),
    State("window", "value"),
    State("long-qty", "value"),
    State("short-qty", "value"),
    prevent_initial_call=True
)


@rate_limiting.limit_backtests(max_requests=10, period=3600)
def run_backtest(n_clicks, ticker, window, long_qty, short_qty):
    # Call the Rust binary with parameters

    try:
        result = subprocess.run(
            ["./trading", ticker, str(window), str(long_qty), str(short_qty)],
            capture_output=True,
            text=True,
            check=True
        )

        # Parse the output
        backtest_result = json.loads(result.stdout)

        # Display the results
        return html.Div([
            html.H3("Backtest Results"),
            html.P(f"PnL: ${backtest_result['pnl']:.2f}"),
            html.P(f"Number of Trades: {backtest_result['n_trades']}"),
        ])
    except subprocess.CalledProcessError as e:
        return html.Div([
            html.H3("Error"),
            html.P(f"Backtest failed: {e.stderr}")
        ])

def run_server():
    return 0
