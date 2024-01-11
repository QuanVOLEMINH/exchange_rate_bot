# Exchange Rate Bot

[![Rust CI](https://github.com/QuanVOLEMINH/exchange_rate_bot/actions/workflows/rust_ci.yml/badge.svg)](https://github.com/QuanVOLEMINH/exchange_rate_bot/actions/workflows/rust_ci.yml)

## Description

This Rust application fetches exchange rates and sends updates to a specified Discord channel. It uses the ExchangeRate-API for fetching current exchange rates and Discord webhooks to post messages in a Discord channel.

## Features

- Fetches daily exchange rates from ExchangeRate-API.
- Sends exchange rate updates to a Discord channel.
- Configurable via environment variables.
