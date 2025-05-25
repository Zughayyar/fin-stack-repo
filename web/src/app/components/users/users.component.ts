import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';
import { FormsModule, ReactiveFormsModule, FormBuilder, FormGroup, Validators } from '@angular/forms';
import { UserService, User } from '../../services/user.service';

@Component({
  selector: 'app-users',
  templateUrl: './users.component.html',
  standalone: true,
  imports: [CommonModule, RouterModule, FormsModule, ReactiveFormsModule]
})
export class UsersComponent implements OnInit {
  users: User[] = [];
  userForm: FormGroup;
  isEditing = false;
  selectedUserId: string | null = null;

  constructor(
    private userService: UserService,
    private fb: FormBuilder
  ) {
    this.userForm = this.fb.group({
      first_name: ['', [Validators.required, Validators.minLength(2)]],
      last_name: ['', [Validators.required, Validators.minLength(2)]],
      email: ['', [Validators.required, Validators.email]],
      password: ['', [Validators.required, Validators.minLength(6)]]
    });
  }

  ngOnInit(): void {
    this.loadUsers();
  }

  loadUsers(): void {
    this.userService.getAllUsers().subscribe(users => {
      this.users = users;
    });
  }

  onSubmit(): void {
    if (this.userForm.valid) {
      if (this.isEditing && this.selectedUserId) {
        this.userService.updateUser(this.selectedUserId, this.userForm.value).subscribe(() => {
          this.loadUsers();
          this.resetForm();
        });
      } else {
        this.userService.createUser(this.userForm.value).subscribe(() => {
          this.loadUsers();
          this.resetForm();
        });
      }
    }
  }

  editUser(user: User): void {
    this.isEditing = true;
    this.selectedUserId = user.id;
    this.userForm.patchValue({
      first_name: user.first_name,
      last_name: user.last_name,
      email: user.email,
      password: '' // Don't populate password for security
    });
  }

  deleteUser(userId: string): void {
    if (confirm('Are you sure you want to delete this user?')) {
      this.userService.deleteUser(userId).subscribe(() => {
        this.loadUsers();
      });
    }
  }

  resetForm(): void {
    this.isEditing = false;
    this.selectedUserId = null;
    this.userForm.reset();
  }

  getErrorMessage(controlName: string): string {
    const control = this.userForm.get(controlName);
    if (control?.hasError('required')) {
      return 'This field is required';
    }
    if (control?.hasError('email')) {
      return 'Please enter a valid email';
    }
    if (control?.hasError('minlength')) {
      return `Minimum length is ${control.errors?.['minlength'].requiredLength} characters`;
    }
    return '';
  }
} 