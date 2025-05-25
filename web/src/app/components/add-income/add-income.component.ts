import { Component, EventEmitter, Output } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { IncomeService, CreateIncomeRequestBody } from '../../services/income.service';

@Component({
  selector: 'app-add-income',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './add-income.component.html'
})
export class AddIncomeComponent {
  @Output() incomeAdded = new EventEmitter<void>();
  @Output() closed = new EventEmitter<void>();

  isOpen = false;
  income: CreateIncomeRequestBody = {
    user_id: '',
    source: '',
    amount: '',
    date: '',
    description: ''
  };

  constructor(private incomeService: IncomeService) {}

  open(userId: string) {
    console.log('Opening add income dialog with user ID:', userId);
    this.resetForm();
    this.income.user_id = userId;
    console.log('Income object after setting user ID:', this.income);
    this.isOpen = true;
  }

  close() {
    this.isOpen = false;
    this.resetForm();
    this.closed.emit();
  }

  private resetForm() {
    this.income = {
      user_id: '',
      source: '',
      amount: '',
      date: '',
      description: ''
    };
  }

  onSubmit() {
    // Format the data according to backend requirements
    const formattedIncome: CreateIncomeRequestBody = {
      user_id: this.income.user_id,
      source: this.income.source.trim(),
      amount: this.income.amount.toString(),
      date: this.income.date,
      description: this.income.description?.trim() || undefined
    };

    console.log('Submitting income with formatted data:', formattedIncome);

    this.incomeService.createIncome(formattedIncome).subscribe({
      next: () => {
        this.incomeAdded.emit();
        this.close();
      },
      error: (error) => {
        console.error('Error adding income:', error);
        // You might want to show an error message to the user here
      }
    });
  }
} 