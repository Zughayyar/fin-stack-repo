import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule, ActivatedRoute } from '@angular/router';
import { FormsModule } from '@angular/forms';
import { ExpenseService, Expense, NewExpense, UpdateExpense } from '../../services/expense.service';
import { IncomeService, Income, NewIncome, UpdateIncome } from '../../services/income.service';
import { User, UserWithIncomes, UserService } from '../../services/user.service';
import { AddIncomeComponent } from '../add-income/add-income.component';
import { AddExpenseComponent } from '../add-expense/add-expense.component';

@Component({
  selector: 'app-dashboard',
  standalone: true,
  imports: [CommonModule, RouterModule, FormsModule, AddIncomeComponent, AddExpenseComponent],
  templateUrl: './dashboard.component.html'
})
export class DashboardComponent implements OnInit {
  currentUser: User | null = null;
  expenses: Expense[] = [];
  incomes: Income[] = [];
  selectedMonth: Date = new Date();
  totalExpenses: number = 0;
  totalIncome: number = 0;
  balance: number = 0;

  // Edit state
  editingExpense: Expense | null = null;
  editingIncome: Income | null = null;
  showEditExpenseModal = false;
  showEditIncomeModal = false;
  editExpenseForm: Partial<Expense> = {};
  editIncomeForm: Partial<Income> = {};

  constructor(
    private expenseService: ExpenseService,
    private incomeService: IncomeService,
    private userService: UserService,
    private route: ActivatedRoute
  ) {}

  ngOnInit() {
    this.route.params.subscribe(params => {
      const userId = params['userId'];
      console.log('Loading user data for ID from route:', userId);
      if (userId) {
        this.loadUserData(userId);
      } else {
        console.error('No user ID found in route parameters');
      }
    });
  }

  loadUserData(userId: string) {
    this.userService.getUserById(userId).subscribe({
      next: (user: UserWithIncomes) => {
        console.log('User data loaded:', user);
        this.currentUser = user;
        this.incomes = user.incomes || [];
        this.loadExpenses(userId);
      },
      error: (error) => {
        console.error('Error loading user:', error);
      }
    });
  }

  loadExpenses(userId: string) {
    this.expenseService.getExpensesByUserId(userId).subscribe({
      next: (expenses) => {
        this.expenses = expenses;
        this.calculateTotals();
      },
      error: (error) => {
        console.error('Error loading expenses:', error);
      }
    });
  }

  calculateTotals() {
    this.totalExpenses = this.expenses.reduce((sum, expense) => sum + Number(expense.amount), 0);
    this.totalIncome = this.incomes.reduce((sum, income) => sum + Number(income.amount), 0);
    this.balance = this.totalIncome - this.totalExpenses;
  }

  changeMonth(delta: number) {
    this.selectedMonth = new Date(
      this.selectedMonth.getFullYear(),
      this.selectedMonth.getMonth() + delta,
      1
    );
    // You might want to reload the data for the new month here
  }

  onIncomeAdded() {
    if (this.currentUser) {
      this.loadUserData(this.currentUser.id);
    }
  }

  onExpenseAdded() {
    if (this.currentUser) {
      this.loadExpenses(this.currentUser.id);
    }
  }

  // Edit Expense
  startEditExpense(expense: Expense) {
    this.editingExpense = { ...expense };
    this.editExpenseForm = { ...expense };
    this.showEditExpenseModal = true;
  }

  saveEditExpense() {
    if (!this.editingExpense) return;
    const update: UpdateExpense = {
      item_name: this.editExpenseForm.item_name,
      amount: this.editExpenseForm.amount,
      date: this.editExpenseForm.date,
      description: this.editExpenseForm.description
    };
    this.expenseService.updateExpense(this.editingExpense.id, update).subscribe({
      next: () => {
        if (this.currentUser) this.loadExpenses(this.currentUser.id);
        this.cancelEditExpense();
      },
      error: (error) => {
        console.error('Error updating expense:', error);
      }
    });
  }

  cancelEditExpense() {
    this.editingExpense = null;
    this.editExpenseForm = {};
    this.showEditExpenseModal = false;
  }

  deleteExpense(expense: Expense) {
    if (confirm('Are you sure you want to delete this expense?')) {
      this.expenseService.deleteExpense(expense.id).subscribe({
        next: () => {
          if (this.currentUser) this.loadExpenses(this.currentUser.id);
        },
        error: (error) => {
          console.error('Error deleting expense:', error);
        }
      });
    }
  }

  // Edit Income
  startEditIncome(income: Income) {
    this.editingIncome = { ...income };
    this.editIncomeForm = { ...income };
    this.showEditIncomeModal = true;
  }

  saveEditIncome() {
    if (!this.editingIncome) return;
    const update: UpdateIncome = {
      source: this.editIncomeForm.source,
      amount: this.editIncomeForm.amount,
      date: this.editIncomeForm.date,
      description: this.editIncomeForm.description
    };
    this.incomeService.updateIncome(this.editingIncome.id, update).subscribe({
      next: () => {
        if (this.currentUser) this.loadUserData(this.currentUser.id);
        this.cancelEditIncome();
      },
      error: (error) => {
        console.error('Error updating income:', error);
      }
    });
  }

  cancelEditIncome() {
    this.editingIncome = null;
    this.editIncomeForm = {};
    this.showEditIncomeModal = false;
  }

  deleteIncome(income: Income) {
    if (confirm('Are you sure you want to delete this income?')) {
      this.incomeService.deleteIncome(income.id).subscribe({
        next: () => {
          if (this.currentUser) this.loadUserData(this.currentUser.id);
        },
        error: (error) => {
          console.error('Error deleting income:', error);
        }
      });
    }
  }
} 