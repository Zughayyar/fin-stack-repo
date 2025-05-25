import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';

export interface Income {
  id: string;
  user_id: string;
  source: string;
  amount: string;
  date: string;
  description?: string;
  created_at: string;
  updated_at: string;
}

// New/Updated Interfaces based on OpenAPI spec
export interface GetAllIncomeBody {
  user_id: string;
}

export interface CreateIncomeRequestBody { // Replaces CreateIncomeRequest
  user_id: string;
  source: string;
  amount: string;
  date: string;
  description?: string;
}

export interface IncomeActionBody {
  user_id: string;
  income_id: string;
}

export interface UpdateIncomeRequestBody { // Replaces Partial<CreateIncomeRequest> for update
  user_id: string;
  income_id: string;
  source?: string;
  amount?: string;
  date?: string;
  description?: string;
}

@Injectable({
  providedIn: 'root'
})
export class IncomeService {
  private apiUrl = environment.apiUrl;

  constructor(private http: HttpClient) {}

  // GET /api/income
  getIncomes(body: GetAllIncomeBody): Observable<Income[]> {
    return this.http.post<Income[]>(`${this.apiUrl}/api/income/get-all`, body);
  }

  // POST /api/income
  createIncome(income: CreateIncomeRequestBody): Observable<Income> {
    return this.http.post<Income>(`${this.apiUrl}/api/income/create`, income);
  }

  // POST /api/income/actions (for get by ID)
  getIncomeById(body: IncomeActionBody): Observable<Income> {
    return this.http.post<Income>(`${this.apiUrl}/api/income/get-by-id`, body);
  }

  // PATCH /api/income/actions (for update)
  updateIncome(income: UpdateIncomeRequestBody): Observable<Income> {
    return this.http.patch<Income>(`${this.apiUrl}/api/income/update`, income);
  }

  // DELETE /api/income/actions (for delete)
  deleteIncome(body: IncomeActionBody): Observable<void> {
    return this.http.request<void>('DELETE', `${this.apiUrl}/api/income/delete`, { body });
  }
} 