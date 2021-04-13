# cti

Rust crates for working with threat intelligence in the STIX 2.0 format.
The goal of this project is to make working with threat intelligence easy and type-safe in Rust.

This project is still very much a work-in-progress.

## stix

The `stix` crate provides the standards-defined functionality for STIX, including core data types and domain objects.
This crate also exposes proc-macros for extending the STIX schema with custom domain objects.

## attck

A consumer of the `stix` crate with additional resources for MITRE matrices and tactics.
