import { useFetchBaseURL } from "./fetch";

export async function createUser(username, password) {
  const { data: resp, error: error } = await useFetchBaseURL("/api/v1/users", {
    method: "POST",
    body: {
      user: {
        username: username,
        password: password,
      },
    },
  });

  if (error.value) {
    let error_message = "unknown";

    if (error.value.statusCode === 422) {
      if (error.value.data.errors.username) {
        if (!error.value.data.errors.username === "taken") {
          error_message = "username_taken";
        } else {
          error_message = "username_length";
        }
      } else if (error.value.data.errors.password) {
        error_message = "password_length";
      }
    }

    return { user: null, error: error_message };
  }

  return { user: resp.value.user, error: null };
}

export async function loginUser(username, password) {
  const { data: resp, error: error } = await useFetchBaseURL(
    "/api/v1/users/login",
    {
      method: "POST",
      body: {
        user: {
          username: username,
          password: password,
        },
      },
    }
  );

  if (error.value) {
    let error_message = "";

    if (error.value.statusCode === 422) {
      if (error.value.data.errors.username_or_password) {
        error_message = "username_or_password_invalid";
      } else {
        error_message = "unknown";
      }
    } else {
      error_message = "unknown";
    }

    return { user: null, error: error_message };
  }

  return { user: resp.value.user, error: null };
}

export async function getCurrentUser(token) {
  const { data: resp } = await useFetchBaseURL("/api/v1/user/whoami", {
    headers: {
      Authorization: `Token ${token}`,
    },
  });

  if (!resp.value) {
    return { user: null, error: "forbidden" };
  }

  return { user: resp.value.user, error: null };
}

export async function getProfile(username) {
  const { data: resp } = await useFetchBaseURL(
    `/api/v1/user?username=${username}`,
    {
      method: "GET",
    }
  );

  if (!resp.value) {
    return { profile: null, error: "not_found" };
  }

  return { profile: resp.value.user, error: null };
}

export async function updateUser(token, image_url) {
  const { data: resp } = await useFetchBaseURL("/api/v1/user", {
    method: "PUT",
    headers: {
      Authorization: `Token ${token}`,
    },
    body: {
      user: {
        image: image_url,
      },
    },
  });

  if (!resp.value) {
    return { user: null, error: "unknown" };
  }

  return { user: resp.value, error: null };
}

export async function uploadPaste(token, filename, type, content) {
  const form = new FormData();
  form.append("content", content);
  form.append("filename", filename);
  form.append("type", type);
  form.append("mime", content.type);

  const { data: paste, error: error } = await useFetchBaseURL(
    "/api/v1/pastes",
    {
      method: "POST",
      headers: {
        Authorization: `Token ${token}`,
      },
      body: form,
    }
  );

  if (error.value) {
    let error_message = "unknown";

    if ((error.value.statusCode = 422)) {
      if (error.value.data.errors.filename) {
        return { paste: null, error: "filename_length" };
      }
    }

    return { paste: null, error: error_message };
  }

  return { paste: paste.value, error: null };
}

export async function downloadPaste(link, filename) {
  if ((link === "", filename === "")) {
    return { paste: null, error: "not_found" };
  }

  const { data: resp } = await useFetchBaseURL(
    `/api/v1/pastes/${link}/${filename}`,
    {
      method: "GET",
    }
  );

  if (!resp.value) {
    return { paste: null, error: "not_found" };
  }

  return { paste: resp, error: null };
}

export async function getPaste(link) {
  if (link === "") {
    return { paste: null, error: "not_found" };
  }

  const { data: resp, error: error } = await useFetchBaseURL(
    `/api/v1/pastes/${link}`,
    {
      method: "GET",
    }
  );

  if (error.value) {
    return { paste: null, error: "not_found" };
  }

  return { paste: resp, error: null };
}

export async function deletePaste(token, link) {
  if (link === "") {
    return { paste: null, error: "not_found" };
  }

  const { error: error } = await useFetchBaseURL(`/api/v1/pastes/${link}`, {
    method: "DELETE",
    headers: {
      Authorization: `Token ${token}`,
    },
  });

  if (error.value) {
    let error_message = "unknown";

    if (error.value.data.errors.file) {
      if (!error.value.data.errors.file === "not_found") {
        error_message = "file_not_found";
      } else {
        error_message = "s3_delete_failed";
      }
    } else if (error.value.data.errors.paste) {
      if (error.value.data.errors.paste === "not_found") {
        error_message = "paste_not_found";
      } else {
        error_message = "db_delete_failed";
      }
    }

    return { error: error_message };
  }

  return { error: null };
}

export async function getPastes(owner, limit, offset = 0) {
  const { data: resp, error: error } = await useFetchBaseURL(
    `/api/v1/pastes?params&owner=${owner}&limit=${limit}&offset=${offset}`,
    {
      method: "GET",
    }
  );

  if (error.value) {
    return { pastes: null, error: "unprocessable" };
  }

  return { pastes: resp.value, error: null };
}
