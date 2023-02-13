package main

import (
	"io/ioutil"
	"log"
	"net/http"

	"github.com/gin-gonic/gin"
	"xilefmusics.de/money-app/helper"
	"xilefmusics.de/money-app/history"
	"xilefmusics.de/money-app/import_transactions"
	"xilefmusics.de/money-app/transaction"
)

func GetTransactions(gc *gin.Context) {
	user, err := helper.GC2User(gc)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	yearFilter := helper.ParseNumber(gc.DefaultQuery("year", "*"), -1, false)
	monthFilter := helper.ParseNumber(gc.DefaultQuery("month", "*"), -1, false)
	podFilter := gc.DefaultQuery("pod", "*")
	debtFilter := gc.DefaultQuery("debt", "*")
	budgetFilter := gc.DefaultQuery("budget", "*")
	inbudgetFilter := gc.DefaultQuery("inbudget", "*")
	typeFilter := gc.DefaultQuery("type", "*")
	idFilter := gc.DefaultQuery("id", "*")

	transactions := transaction.Filter(globalData.GetTransactions(user), yearFilter, monthFilter, podFilter, debtFilter, budgetFilter, inbudgetFilter, typeFilter, idFilter)

	gc.IndentedJSON(http.StatusOK, transactions)
}

func Lint(gc *gin.Context) {
	user, err := helper.GC2User(gc)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	transactions := globalData.GetTransactions(user)
	lint := transaction.LintTransactions(transactions)

	gc.IndentedJSON(http.StatusOK, lint)
}

func Reindex(gc *gin.Context) {
	user, err := helper.GC2User(gc)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	globalData.Reindex(user)

	gc.IndentedJSON(http.StatusOK, "")
}

func GetPods(gc *gin.Context) {
	user, err := helper.GC2User(gc)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	transactions := globalData.GetTransactions(user)
	pods := transaction.GetPods(transactions)

	gc.IndentedJSON(http.StatusOK, pods)
}

func GetDebts(gc *gin.Context) {
	user, err := helper.GC2User(gc)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	transactions := globalData.GetTransactions(user)
	debts := transaction.GetDebts(transactions)

	gc.IndentedJSON(http.StatusOK, debts)
}

func GetBudgets(gc *gin.Context) {
	user, err := helper.GC2User(gc)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	transactions := globalData.GetTransactions(user)
	budgets := transaction.GetBudgets(transactions)

	gc.IndentedJSON(http.StatusOK, budgets)
}

func GetInbudgets(gc *gin.Context) {
	user, err := helper.GC2User(gc)
	if err != nil {
		log.Printf("ERROR in GetTransactions: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	transactions := globalData.GetTransactions(user)
	inbudget := transaction.GetInbudgets(transactions)

	gc.IndentedJSON(http.StatusOK, inbudget)
}

func GetTags(gc *gin.Context) {
	user, err := helper.GC2User(gc)
	if err != nil {
		log.Printf("ERROR in GetTransactions: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	transactions := globalData.GetTransactions(user)
	tags := transaction.GetTags(transactions)

	gc.IndentedJSON(http.StatusOK, tags)
}

func GetHistory(gc *gin.Context) {
	user, err := helper.GC2User(gc)
	if err != nil {
		log.Printf("ERROR in GetTransactions: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	year := helper.ParseNumber(gc.DefaultQuery("year", "0"), 0, true)
	month := helper.ParseNumber(gc.DefaultQuery("month", "0"), 0, true)
	day := helper.ParseNumber(gc.DefaultQuery("day", "0"), 0, true)
	len := helper.ParseNumber(gc.DefaultQuery("len", "1"), 1, true)

	if day == 0 && month == 0 && year == 0 {
		year = 1
	}

	transactions := globalData.GetTransactions(user)

	switch gc.Param("kind") {
	case "debt":
		gc.IndentedJSON(http.StatusOK, history.History(int(year), int(month), int(day), int(len), history.Debt, transactions))
		break
	case "budget":
		gc.IndentedJSON(http.StatusOK, history.History(int(year), int(month), int(day), int(len), history.Budget, transactions))
		break
	case "inbudget":
		gc.IndentedJSON(http.StatusOK, history.History(int(year), int(month), int(day), int(len), history.Inbudget, transactions))
		break
	case "pod":
		gc.IndentedJSON(http.StatusOK, history.History(int(year), int(month), int(day), int(len), history.Pod, transactions))
		break
	case "wealth":
		gc.IndentedJSON(http.StatusOK, history.History(int(year), int(month), int(day), int(len), history.Wealth, transactions))
		break
	default:
		gc.IndentedJSON(http.StatusNotFound, "404 page not found")
	}
}

func PostTransactions(gc *gin.Context) {
	user, err := helper.GC2User(gc)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	body, err := ioutil.ReadAll(gc.Request.Body)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	transactions, err := import_transactions.Import(string(body))
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	newTransactions := globalData.AddTransactions(user, transactions, true)

	gc.IndentedJSON(http.StatusOK, newTransactions)
}

func UpdateTransactions(gc *gin.Context) {
	user, err := helper.GC2User(gc)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	body, err := ioutil.ReadAll(gc.Request.Body)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	transactions, err := import_transactions.Import(string(body))
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	err = globalData.UpdateTransactions(user, transactions, true)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	gc.IndentedJSON(http.StatusOK, transactions)
}

func DeleteTransactions(gc *gin.Context) {
	user, err := helper.GC2User(gc)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	body, err := ioutil.ReadAll(gc.Request.Body)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	transactions, err := import_transactions.Import(string(body))
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	deletedTransactions := globalData.DeleteTransactions(user, transactions, true)

	gc.IndentedJSON(http.StatusOK, deletedTransactions)
}

func Undo(gc *gin.Context) {
	user, err := helper.GC2User(gc)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	globalData.Undo(user)

	gc.IndentedJSON(http.StatusOK, "")
}
