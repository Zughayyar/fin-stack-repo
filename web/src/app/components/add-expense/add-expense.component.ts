import { Component, EventEmitter, Output } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ExpenseService, CreateExpenseRequestBody } from '../../services/expense.service';

@Component({
  selector: 'app-add-expense',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './add-expense.component.html'
})
export class AddExpenseComponent {
  @Output() expenseAdded = new EventEmitter<void>();
  @Output() closed = new EventEmitter<void>();

  isOpen = false;
  expense: CreateExpenseRequestBody = {
    user_id: '',
    item_name: '',
    amount: '',
    date: '',
    description: ''
  };

  constructor(private expenseService: ExpenseService) {}

  open(userId: string) {
    console.log('Opening add expense dialog with user ID:', userId);
    this.resetForm();
    this.expense.user_id = userId;
    console.log('Expense object after setting user ID:', this.expense);
    this.isOpen = true;
  }

  close() {
    this.isOpen = false;
    this.resetForm();
    this.closed.emit();
  }

  private resetForm() {
    this.expense = {
      user_id: '',
      item_name: '',
      amount: '',
      date: '',
      description: ''
    };
  }

  onSubmit() {
    const formattedExpense: CreateExpenseRequestBody = {
      user_id: this.expense.user_id,
      item_name: this.expense.item_name.trim(),
      amount: this.expense.amount.toString(),
      date: this.expense.date,
      description: this.expense.description?.trim() || undefined
    };

    console.log('Submitting expense with formatted data:', formattedExpense);

    this.expenseService.createExpense(formattedExpense).subscribe({
      next: () => {
        this.expenseAdded.emit();
        this.close();
      },
      error: (error) => {
        console.error('Error adding expense:', error);
      }
    });
  }
} 