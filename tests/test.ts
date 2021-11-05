// POST http://localhost:4300/user/register
// Content-Type: application/json
//
// {
//   "username" : "test1002",
//   "password" : "test1002",
//   "email" : "email"
// }

for (let i = 0; i < 20; i++) {
  fetch('http://localhost:4300/user/register', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      username: `test${1000 + i}`,
      password: `test${1000 + i}`,
      email: `email${1000 + i}@qq.com`
    })
  }).then((res) => {
    console.log(res);
  });
}
