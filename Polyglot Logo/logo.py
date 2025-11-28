#!/usr/bin/env python3
"""
Polyglot Logo - Terminal Renderer using Rich
Draws the Polyglot logo with properly scaled symbols
"""

from rich.console import Console
from rich import box

console = Console()


def draw_logo_best():
    """Best version with properly scaled symbols"""

    # Top green box with large dot
    console.print("\n[green]        ┌─────────────┐[/green]")
    console.print("[green]        │             │[/green]")
    console.print("[green]        │    ▄▄▄▄▄    │[/green]")
    console.print("[green]        │   ███████   │[/green]")
    console.print("[green]        │   ███████   │[/green]")
    console.print("[green]        │    ▀▀▀▀▀    │[/green]")
    console.print("[green]        │             │[/green]")
    console.print("[green]        └─────────────┘[/green]")

    # Blue box with chevron          Red box with exclamation
    console.print("[blue]   ┌─────────────┐[/blue]  [red]┌─────────────┐[/red]")
    console.print("[blue]   │             │[/blue]  [red]│             │[/red]")
    console.print("[blue]   │  [green]▀▀▄[/green]        │[/blue]  [red]│    ▄███▄    │[/red]")
    console.print("[blue]   │  [green]▀▀▀▄▄[/green]      │[/blue]  [red]│    █████    │[/red]")
    console.print("[blue]   │  [green]▀▀▀▀▀▄▄[/green]    │[/blue]  [red]│    █████    │[/red]")
    console.print("[blue]   │  [green]▀▀▀▄▄[/green]      │[/blue]  [red]│             │[/red]")
    console.print("[blue]   │  [green]▀▀▄[/green]        │[/blue]  [red]│    ▄███▄    │[/red]")
    console.print("[blue]   │             │[/blue]  [red]│             │[/red]")
    console.print("[blue]   └─────────────┘[/blue]  [red]└─────────────┘[/red]")

    console.print("\n      [bold red]Poly[/bold red][bold cyan]glot[/bold cyan]\n")


def draw_logo_blocks():
    """Version using solid block characters"""

    # Top green box with filled dot
    console.print("\n[green]        ╔═════════════╗[/green]")
    console.print("[green]        ║             ║[/green]")
    console.print("[green]        ║    █████    ║[/green]")
    console.print("[green]        ║    █████    ║[/green]")
    console.print("[green]        ║    █████    ║[/green]")
    console.print("[green]        ║             ║[/green]")
    console.print("[green]        ╚═════════════╝[/green]")

    # Blue box with chevron          Red box with exclamation
    console.print("[blue]   ╔═════════════╗[/blue]  [red]╔═════════════╗[/red]")
    console.print("[blue]   ║             ║[/blue]  [red]║             ║[/red]")
    console.print("[blue]   ║   [green]██[/green]        ║[/blue]  [red]║    █████    ║[/red]")
    console.print("[blue]   ║   [green]████[/green]      ║[/blue]  [red]║    █████    ║[/red]")
    console.print("[blue]   ║   [green]██████[/green]    ║[/blue]  [red]║    █████    ║[/red]")
    console.print("[blue]   ║   [green]████[/green]      ║[/blue]  [red]║             ║[/red]")
    console.print("[blue]   ║   [green]██[/green]        ║[/blue]  [red]║    █████    ║[/red]")
    console.print("[blue]   ║             ║[/blue]  [red]║             ║[/red]")
    console.print("[blue]   ╚═════════════╝[/blue]  [red]╚═════════════╝[/red]")

    console.print("\n      [bold red]Poly[/bold red][bold cyan]glot[/bold cyan]\n")


def draw_logo_minimal():
    """Minimal clean version"""

    console.print()
    console.print("[green]          ▄▄▄▄▄▄▄▄▄▄▄▄▄[/green]")
    console.print("[green]          █           █[/green]")
    console.print("[green]          █   █████   █[/green]")
    console.print("[green]          █   █████   █[/green]")
    console.print("[green]          █           █[/green]")
    console.print("[green]          ▀▀▀▀▀▀▀▀▀▀▀▀▀[/green]")

    console.print("[blue]     ▄▄▄▄▄▄▄▄▄▄▄▄▄[/blue]  [red]▄▄▄▄▄▄▄▄▄▄▄▄▄[/red]")
    console.print("[blue]     █           █[/blue]  [red]█           █[/red]")
    console.print("[blue]     █  [green]▀▄[/green]      █[/blue]  [red]█   █████   █[/red]")
    console.print("[blue]     █  [green]▀▀▄▄[/green]    █[/blue]  [red]█   █████   █[/red]")
    console.print("[blue]     █  [green]▀▀▀▄▄[/green]  █[/blue]  [red]█           █[/red]")
    console.print("[blue]     █  [green]▀▀▄▄[/green]    █[/blue]  [red]█   █████   █[/red]")
    console.print("[blue]     █  [green]▀▄[/green]      █[/blue]  [red]█           █[/red]")
    console.print("[blue]     █           █[/blue]  [red]█           █[/red]")
    console.print("[blue]     ▀▀▀▀▀▀▀▀▀▀▀▀▀[/blue]  [red]▀▀▀▀▀▀▀▀▀▀▀▀▀[/red]")

    console.print("\n        [bold red]Poly[/bold red][bold cyan]glot[/bold cyan]\n")


def main():
    console.print("\n[bold]═══ Polyglot Logo ═══[/bold]\n")

    console.print("[yellow]Version 1: Unicode Borders[/yellow]")
    draw_logo_best()

    console.print("─" * 50)
    console.print("\n[yellow]Version 2: Double-Line Borders[/yellow]")
    draw_logo_blocks()

    console.print("─" * 50)
    console.print("\n[yellow]Version 3: Minimal Blocks[/yellow]")
    draw_logo_minimal()


if __name__ == "__main__":
    main()