package main

import (
	"fmt"
	"log"
	"os"

	"github.com/ReidMason/pleco/internal/colours"
	filehandler "github.com/ReidMason/pleco/internal/file_handler"
	"github.com/ReidMason/pleco/internal/list"
	"github.com/charmbracelet/bubbles/viewport"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/joho/godotenv"
)

type Model struct {
	selectedDir string
	logs        string
	viewport    viewport.Model
	list        list.Model
	index       int
	ready       bool
}

func (m Model) Init() tea.Cmd {
	return nil
}

func (m Model) executeAction(idx int) (Model, tea.Cmd) {
	switch idx {
	case 0:
		files := filehandler.GetFiles(m.selectedDir)
		summary := filehandler.GetFilesSummary(files)

		commonFiles := ""
		for i, fileType := range summary.CommonTypes {
			commonFiles += fmt.Sprintf("    %d: %s (%d)\n", i+1, fileType.Filetype, fileType.Count)
		}

		m.logs += fmt.Sprintf("Total files: %d\nCommon filetypes:\n%s\n\n", summary.Filecount, commonFiles)
		m.viewport.SetContent(m.logs)
		m.viewport.GotoBottom()
	case 1:
		return m, tea.Quit
	}

	return m, nil
}

func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	var (
		cmd  tea.Cmd
		cmds []tea.Cmd
	)

	m.list, cmd = m.list.Update(msg)
	cmds = append(cmds, cmd)

	switch msg := msg.(type) {
	case tea.KeyMsg:
		if msg.String() == "ctrl+c" {
			return m, tea.Quit
		} else if msg.String() == "enter" {
			m, cmd = m.executeAction(m.list.Index())
			return m, cmd
		}
	case tea.WindowSizeMsg:
		if !m.ready {
			m.viewport = viewport.New(msg.Width, msg.Height/2)
			m.viewport.SetContent(m.logs)
			m.ready = true
		} else {
			m.viewport.Width = msg.Width
			m.viewport.Height = msg.Height
		}
	}

	m.viewport, cmd = m.viewport.Update(msg)
	cmds = append(cmds, cmd)

	return m, tea.Batch(cmds...)
}

var BorderStyle = lipgloss.
	NewStyle().
	Border(lipgloss.RoundedBorder()).
	BorderForeground(lipgloss.Color(colours.Blue))

var HeaderStyle = lipgloss.
	NewStyle().
	Background(lipgloss.Color(colours.Flamingo)).
	Foreground(lipgloss.Color(colours.Crust)).
	Align(lipgloss.Center)

func (m Model) View() string {
	title := HeaderStyle.Copy().
		Background(lipgloss.Color(colours.Mauve)).
		Width(m.viewport.Width).
		Render("Pleco")

	directoryInfo := fmt.Sprintf("\nSelected directory: '%s'\n", m.selectedDir)
	options := "Options:" + m.list.Render()
	output := lipgloss.JoinVertical(lipgloss.Left, title, directoryInfo, options)

	logsTitle := HeaderStyle.
		Width(m.viewport.Width).
		Render("Logs")

	logs := BorderStyle.
		Width(m.viewport.Width - BorderStyle.GetHorizontalBorderSize()).
		Render(m.viewport.View())

	output = lipgloss.JoinVertical(lipgloss.Top, output, logsTitle, logs)

	return output
}

func main() {
	debug := false
	err := godotenv.Load()
	if err == nil {
		debug = "true" == os.Getenv("DEBUG_GIT_UI")
	}

	if debug {
		f, err := tea.LogToFile("debug.log", "debug")
		if err != nil {
			fmt.Println("fatal:", err)
			os.Exit(1)
		}
		defer f.Close()
	}

	items := []list.Item{
		list.NewItem("Get file count", "Count all files"),
		list.NewItem("Quit", "Close the app"),
	}

	currDir, err := os.Getwd()
	if err != nil {
		log.Print("Failed to get current directory")
		return
	}

	m := Model{
		list:        list.New(items),
		index:       0,
		selectedDir: currDir,
		logs:        "",
	}

	p := tea.NewProgram(m, tea.WithAltScreen(), tea.WithMouseCellMotion())

	if _, err := p.Run(); err != nil {
		fmt.Println("Error running program:", err)
		os.Exit(1)
	}
}
