# libredfish2

[![Latest Release](https://gitlab.com/ngreese/libredfish2/-/badges/release.svg)](https://gitlab.com/ngreese/libredfish2/-/releases)

[![pipeline status](https://gitlab.com/ngreese/libredfish2/badges/main/pipeline.svg)](https://gitlab.com/ngreese/libredfish2/-/commits/main)

[![coverage report](https://gitlab.com/ngreese/libredfish2/badges/main/coverage.svg)](https://gitlab.com/ngreese/libredfish2/-/commits/main) 

## Introduction

This library crate is a continuation of the work that the contributers of [libredfish](https://github.com/cholcombe973/libredfish) started.

The initial version was a fork of their repository and all content that is equivalent is attributed to them.

This library serves as an interface to DTMF's Redfish standard for enterprise hardware.

## Redfish

All content of this library follows the Redfish Specification, detailed at [their website](https://www.dmtf.org/standards/redfish)

## Features

There are two features in this crate:
* blocking - All requests to a Redfish endpoint use the `reqwest::blocking::Client` struct. Use this if you want to perform one request at a time. This is the default feature.
* async - Enables asynchronous requests to Redfish endpoints. Uses the regular `reqwest::Client` to perform requests.