package main

import (
	"log"
	"net/http"

	"github.com/gin-gonic/gin"

	"xilefmusics.de/money-app/data"
)

var globalData data.Data

func main() {
	d, err := data.New()
	if err != nil {
		log.Fatal(err.Error())
	}
	globalData = d

	router := gin.New()
	router.GET("/transactions", GetTransactions)
	router.GET("/pods", GetPods)
	router.GET("/debts", GetDebts)
	router.GET("/budgets", GetBudgets)
	router.GET("/inbudgets", GetInbudgets)
	router.GET("/history/:kind", GetHistory)
	log.Fatal(http.ListenAndServe(":8080", router))
}
