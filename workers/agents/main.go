package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	fmt.Println("========================================")
	fmt.Println("Ochi AI Agent (Go + Rust)")
	fmt.Println("========================================")
	fmt.Println()

	// Check GPU
	if HasGPU() {
		fmt.Println("✅ GPU acceleration available")
	} else {
		fmt.Println("ℹ️  CPU-only mode")
	}
	fmt.Println()

	// Create Rust core
	core := NewRustCore()
	defer core.Close()

	// Load model
	modelPath := "../../models/qwen3.5-0.8b.gguf"
	fmt.Printf("Loading model: %s\n", modelPath)

	err := core.LoadModel(modelPath, 2048, 999, 0)
	if err != nil {
		fmt.Printf("❌ Failed to load model: %v\n", err)
		fmt.Println("Make sure the model file exists and Rust library is built")
		return
	}
	fmt.Println("✅ Model loaded successfully")
	fmt.Println()

	// Interactive chat
	fmt.Println("Enter your prompts (type 'quit' to exit):")
	fmt.Println()

	scanner := bufio.NewScanner(os.Stdin)
	for {
		fmt.Print("You: ")
		if !scanner.Scan() {
			break
		}

		prompt := strings.TrimSpace(scanner.Text())
		if prompt == "" || prompt == "quit" || prompt == "exit" {
			break
		}

		fmt.Print("AI: ")
		response, err := core.Generate(prompt)
		if err != nil {
			fmt.Printf("Error: %v\n", err)
			continue
		}

		fmt.Println(response)
		fmt.Println()
	}

	fmt.Println()
	fmt.Println("Goodbye!")
}
