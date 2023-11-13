package list

import (
	"github.com/ReidMason/pleco/internal/colours"
	"github.com/charmbracelet/bubbles/key"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

type Model struct {
	items []Item
	index int
}

type Item struct {
	title, description string
}

const LinePadding = 2

func NewItem(title, description string) Item {
	return Item{title: title, description: description}
}

func New(items []Item) Model {
	return Model{
		items: items,
		index: 0,
	}
}

func (m Model) Update(msg tea.Msg) (Model, tea.Cmd) {
	keyDown := key.NewBinding(
		key.WithKeys("down", "j"),
		key.WithHelp("down/j", "Down"),
	)

	keyUp := key.NewBinding(
		key.WithKeys("up", "k"),
		key.WithHelp("up/k", "Up"),
	)

	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch {
		case key.Matches(msg, keyDown):
			return m.handleKeyDown(), nil
		case key.Matches(msg, keyUp):
			return m.handleKeyUp(), nil
		}
	}

	return m, nil
}

func (m Model) handleKeyDown() Model {
	m.index = min(len(m.items)-1, m.index+1)
	return m
}

func (m Model) handleKeyUp() Model {
	m.index = max(0, m.index-1)
	return m
}

func (m Model) Render() string {
	options := "\n"
	for i, option := range m.items {
		line := lipgloss.NewStyle().
			PaddingLeft(LinePadding - 1).
			PaddingRight(LinePadding).
			Render(option.title)

		if i == m.index {
			line = lipgloss.NewStyle().
				Background(lipgloss.Color(colours.Lavender)).
				Foreground(lipgloss.Color(colours.Surface0)).
				Render(line)
		}
		options += line + "\n"
	}

	return options
}

func (m Model) Index() int {
	return m.index
}
