const API_URL = '/api';

let bookmarks = [];
let categories = [];
let filteredBookmarks = [];
let currentUser = null;
let authToken = null;

const addBookmarkForm = document.getElementById('addBookmarkForm');
const addCategoryBtn = document.getElementById('addCategoryBtn');
const newCategoryNameInput = document.getElementById('newCategoryName');
const searchInput = document.getElementById('searchInput');
const categoryFilter = document.getElementById('categoryFilter');
const bookmarksList = document.getElementById('bookmarksList');
const categorySelect = document.getElementById('category_id');
const userInfo = document.getElementById('userInfo');
const usernameDisplay = document.getElementById('username');
const logoutBtn = document.getElementById('logoutBtn');

document.addEventListener('DOMContentLoaded', () => {
    checkAuth();
});

function checkAuth() {
    authToken = localStorage.getItem('token');
    const userStr = localStorage.getItem('user');

    if (!authToken || !userStr) {
        window.location.href = 'login.html';
        return;
    }

    currentUser = JSON.parse(userStr);
    displayUserInfo();
    loadCategories();
    loadBookmarks();
    setupEventListeners();
}

function displayUserInfo() {
    if (currentUser) {
        usernameDisplay.textContent = `👤 ${currentUser.username}`;
        userInfo.style.display = 'flex';
    }
}

function setupEventListeners() {
    addBookmarkForm.addEventListener('submit', handleAddBookmark);
    addCategoryBtn.addEventListener('click', handleAddCategory);
    searchInput.addEventListener('input', handleSearch);
    categoryFilter.addEventListener('change', handleFilterByCategory);
    logoutBtn.addEventListener('click', handleLogout);
}

function handleLogout() {
    localStorage.removeItem('token');
    localStorage.removeItem('user');
    window.location.href = 'login.html';
}

async function apiRequest(url, options = {}) {
    try {
        const headers = {
            'Content-Type': 'application/json',
            ...options.headers
        };

        if (authToken) {
            headers['Authorization'] = `Bearer ${authToken}`;
        }

        const response = await fetch(API_URL + url, {
            headers,
            ...options
        });

        if (response.status === 401) {
            localStorage.removeItem('token');
            localStorage.removeItem('user');
            window.location.href = 'login.html';
            return;
        }

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();
        return data;
    } catch (error) {
        console.error('API Error:', error);
        alert('Ошибка при выполнении запроса: ' + error.message);
        throw error;
    }
}

async function loadCategories() {
    try {
        const response = await apiRequest('/categories');
        categories = response.data || [];
        updateCategorySelects();
    } catch (error) {
        console.error('Error loading categories:', error);
        categories = [];
    }
}

async function loadBookmarks() {
    try {
        const response = await apiRequest('/bookmarks');
        bookmarks = response.data || [];
        filteredBookmarks = bookmarks;
        renderBookmarks();
    } catch (error) {
        console.error('Error loading bookmarks:', error);
        bookmarksList.innerHTML = '<p class="no-bookmarks">Ошибка загрузки закладок</p>';
    }
}

function updateCategorySelects() {
    categorySelect.innerHTML = '<option value="">Выберите категорию...</option>';
    categories.forEach(category => {
        const option = document.createElement('option');
        option.value = category.id;
        option.textContent = category.name;
        categorySelect.appendChild(option);
    });

    categoryFilter.innerHTML = '<option value="">Все категории</option>';
    categories.forEach(category => {
        const option = document.createElement('option');
        option.value = category.id;
        option.textContent = category.name;
        categoryFilter.appendChild(option);
    });
}

async function handleAddBookmark(e) {
    e.preventDefault();

    const formData = new FormData(addBookmarkForm);
    const bookmarkData = {
        title: formData.get('title'),
        url: formData.get('url'),
        description: formData.get('description') || null,
        category_id: formData.get('category_id')
    };

    try {
        await apiRequest('/bookmarks', {
            method: 'POST',
            body: JSON.stringify(bookmarkData)
        });

        addBookmarkForm.reset();
        await loadBookmarks();
        alert('Закладка успешно добавлена!');
    } catch (error) {
        console.error('Error adding bookmark:', error);
    }
}

async function handleAddCategory() {
    const categoryName = newCategoryNameInput.value.trim();

    if (!categoryName) {
        alert('Введите название категории');
        return;
    }

    try {
        await apiRequest('/categories', {
            method: 'POST',
            body: JSON.stringify({ name: categoryName })
        });

        newCategoryNameInput.value = '';
        await loadCategories();
        alert('Категория успешно добавлена!');
    } catch (error) {
        console.error('Error adding category:', error);
    }
}

function handleSearch(e) {
    const query = e.target.value.toLowerCase().trim();

    if (!query) {
        filteredBookmarks = bookmarks;
    } else {
        filteredBookmarks = bookmarks.filter(bookmark => {
            return (
                bookmark.title.toLowerCase().includes(query) ||
                bookmark.url.toLowerCase().includes(query) ||
                (bookmark.description && bookmark.description.toLowerCase().includes(query))
            );
        });
    }

    renderBookmarks();
}

function handleFilterByCategory(e) {
    const categoryId = e.target.value;

    if (!categoryId) {
        filteredBookmarks = bookmarks;
    } else {
        filteredBookmarks = bookmarks.filter(bookmark => {
            return bookmark.category && bookmark.category.id === categoryId;
        });
    }

    renderBookmarks();
}

function renderBookmarks() {
    if (!filteredBookmarks || filteredBookmarks.length === 0) {
        bookmarksList.innerHTML = '<p class="no-bookmarks">Нет закладок для отображения</p>';
        return;
    }

    bookmarksList.innerHTML = '';

    filteredBookmarks.forEach(bookmark => {
        const card = createBookmarkCard(bookmark);
        bookmarksList.appendChild(card);
    });
}

function createBookmarkCard(bookmark) {
    const card = document.createElement('div');
    card.className = 'bookmark-card';

    const title = document.createElement('h3');
    title.textContent = bookmark.title;

    const link = document.createElement('a');
    link.href = bookmark.url;
    link.textContent = bookmark.url;
    link.target = '_blank';
    link.rel = 'noopener noreferrer';

    const categoryTag = document.createElement('div');
    categoryTag.className = 'category-tag';
    categoryTag.textContent = `Категория: ${bookmark.category ? bookmark.category.name : 'Неизвестно'}`;

    const deleteBtn = document.createElement('button');
    deleteBtn.className = 'btn btn-danger';
    deleteBtn.textContent = 'Удалить';
    deleteBtn.onclick = () => handleDeleteBookmark(bookmark.id);

    card.appendChild(title);
    card.appendChild(link);

    if (bookmark.description) {
        const description = document.createElement('p');
        description.textContent = bookmark.description;
        card.appendChild(description);
    }

    card.appendChild(categoryTag);
    card.appendChild(deleteBtn);

    return card;
}

async function handleDeleteBookmark(bookmarkId) {
    if (!confirm('Вы уверены, что хотите удалить эту закладку?')) {
        return;
    }

    try {
        await apiRequest(`/bookmarks/${bookmarkId}`, {
            method: 'DELETE'
        });

        await loadBookmarks();
        alert('Закладка успешно удалена!');
    } catch (error) {
        console.error('Error deleting bookmark:', error);
    }
}
