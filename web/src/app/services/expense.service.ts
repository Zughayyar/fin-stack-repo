import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';

export interface Expense {
  id: string;
  user_id: string;
  item_name: string;
  amount: string;
  date: string;
  description?: string;
  created_at: string;
  updated_at: string;
}

// New/Updated Interfaces based on OpenAPI spec
export interface GetAllExpensesBody {
  user_id: string;
}

export interface CreateExpenseRequestBody { // Replaces CreateExpenseRequest
  user_id: string;
  item_name: string;
  amount: string;
  date: string;
  description?: string;
}

export interface ExpenseActionBody {
  user_id: string;
  expense_id: string;
}

export interface UpdateExpenseRequestBody { // Replaces Partial<CreateExpenseRequest> for update
  user_id: string;
  expense_id: string;
  item_name?: string;
  amount?: string;
  date?: string;
  description?: string;
}

@Injectable({
  providedIn: 'root'
})
export class ExpenseService {
  private apiUrl = environment.apiUrl;

  constructor(private http: HttpClient) {}

  // GET /api/expenses
  getExpenses(body: GetAllExpensesBody): Observable<Expense[]> {
    return this.http.post<Expense[]>(`${this.apiUrl}/api/expenses/get-all`, body);
  }

  // POST /api/expenses
  createExpense(expense: CreateExpenseRequestBody): Observable<Expense> {
    return this.http.post<Expense>(`${this.apiUrl}/api/expenses/create`, expense);
  }

  // POST /api/expenses/actions (for get by ID)
  getExpenseById(body: ExpenseActionBody): Observable<Expense> {
    return this.http.post<Expense>(`${this.apiUrl}/api/expenses/get-by-id`, body);
  }

  // PATCH /api/expenses/actions (for update)
  updateExpense(expense: UpdateExpenseRequestBody): Observable<Expense> {
    return this.http.patch<Expense>(`${this.apiUrl}/api/expenses/update`, expense);
  }

  // DELETE /api/expenses/actions (for delete)
  deleteExpense(body: ExpenseActionBody): Observable<void> {
    // For DELETE with body, HttpClient.delete might not work as expected across all browsers/servers.
    // Using http.request for consistency and explicit body handling.
    return this.http.request<void>('DELETE', `${this.apiUrl}/api/expenses/delete`, { body });
  }
} 