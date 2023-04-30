
## API Reference

#### Register user

```http
  POST /user/
```

#### Login user

```http
  POST /user/login
```

#### get all users

```http
  GET /users/
```

#### get user details

```http
  GET /user/{id}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `id`      | `string` | **Required**. User id to get user details |


#### update user details

```http
  PUT /user/{id}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `id`      | `string` | **Required**. User id to update user details |

#### delete user details

```http
  DELETE /user/{id}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `id`      | `string` | **Required**. User id to delete user details |



