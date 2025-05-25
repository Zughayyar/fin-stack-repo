import { Routes } from '@angular/router';
import { UsersComponent } from './components/users/users.component';
import { DashboardComponent } from './components/dashboard/dashboard.component';

export const routes: Routes = [
  { path: '', component: UsersComponent },
  { path: 'dashboard/:userId', component: DashboardComponent },
  { path: '**', redirectTo: '' }
];
