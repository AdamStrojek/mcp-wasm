# Model Context Protocol in WebAssembly Components

> [!WARNING]
> This project is under heavy development, and all interfaces are subject to change at any moment without prior notice. Use with caution in production environments.


## Introduction

This project adapts the **Model Context Protocol (MCP)** to operate within the sandboxed environment of WebAssembly Components. MCP is an open protocol that standardizes how applications provide context to large language models (LLMs).

The original MCP specification focuses on two primary environments:
- **Remote execution**: where the MCP server communicates via WebSocket.
- **Local execution**: where the MCP server runs as a standard program and communicates over standard input/output (stdio).

## Motivation

While the original MCP specification provides a flexible way to deploy context providers, it also introduces potential security concerns. These concerns are highlighted in the official MCP documentation, which notes risks associated with unrestricted access and execution capabilities of MCP servers. Specifically, running an MCP server in its standard environments may:
- permit unlimited network access,
- allow **full disk access**,

Currently, MCP officially supports only three programming languages: Python, TypeScript, and Kotlin. While there are additional unofficial implementations in other languages, the quality of these implementations varies.

## Goals and Benefits

This project proposes a solution to address these security concerns: running the MCP server in sandboxed environment provided by a WebAssembly runtime. This approach provides:
- **A sandbox environment**: This allows fine-grained control over access to computer resources. For example, you can specify what IP addresses are allowed or which files are available at the runtime level. This ensures that downloaded code never has unrestricted access to your computer.
- **Language independence**: Developers can use any programming language that compiles to WebAssembly, significantly expanding flexibility and enabling broader adoption.
- **Backward compatibility with classical MCP servers**: By supporting the WebAssembly System Interface (WASI), this solution can be easily adapted to run as a traditional MCP server, with the added benefits of sandboxing.
- **Portability**: WebAssembly is platform-agnostic, allowing the same MCP server to run seamlessly across different environments.

## What This Repository Contains

This repository contains interface definitions, in the format of WIT files, for the [WebAssembly Component Model](https://github.com/webassembly/component-model). These interfaces aim to provide a compatible API to the original MCP by recreating each of its aspects.

---

### License

This project is licensed under the MIT License.